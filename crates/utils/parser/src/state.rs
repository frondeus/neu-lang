use crate::{Context, Lexer, Parser, TokenKind, NodeId, Arena};
use std::fmt;
use neu_diagnostics::{Diagnostics, Diagnostic};

pub struct State<Tok: TokenKind> {
    lexer: Lexer<Tok>,
    errors: Diagnostics<NodeId>,
    new_errors: Diagnostics<()>,
    nodes: Arena,
}

impl<Tok: TokenKind> State<Tok> {
    fn new(lexer: Lexer<Tok>) -> Self {
        Self {
            lexer,
            errors: Default::default(),
            new_errors: Default::default(),
            nodes: Default::default(),
        }
    }

    // TODO: Ideally this should take ownership but it requires major refactor in the parser.
    //  Especially in the flow of node builder
    pub(crate) fn transform<Tok2>(&mut self) -> State<Tok2>
    where
        Tok2: TokenKind,
        Tok::Extra: Into<Tok2::Extra>,
    {
        let lexer: Lexer<Tok2> = self.lexer.transform();
        State {
            lexer,
            nodes: self.nodes.take(),
            errors: std::mem::take(&mut self.errors),
            new_errors: Default::default(),
        }
    }

    pub(crate) fn restore<Tok2>(&mut self, mut other: State<Tok2>)
    where
        Tok2: TokenKind,
        Tok2::Extra: Into<Tok::Extra>,
    {
        let lexer: Lexer<Tok> = other.lexer().transform();
        self.lexer = lexer;
        self.errors = std::mem::take(&mut other.errors);
        self.nodes = other.nodes.take();
    }

    pub fn nodes(&mut self) -> &mut Arena {
        &mut self.nodes
    }

    pub fn lexer(&self) -> &Lexer<Tok> {
        &self.lexer
    }

    pub fn lexer_mut(&mut self) -> &mut Lexer<Tok> {
        &mut self.lexer
    }

    pub fn error(&mut self, e: Diagnostic) {
        self.new_errors.push(((), e));
    }

    pub fn commit_errors(&mut self, id: NodeId) {
        self.errors
            .extend(self.new_errors.drain(..).map(|(_, e)| (id, e)));
    }

    pub fn parse(lexer: Lexer<Tok>, parser: impl Parser<Tok>) -> ParseResult {
        let mut state = Self::new(lexer);
        let ctx = Context::default();
        let root = parser.parse(&mut state, &ctx);
        let root = state.nodes().add(root);
        let nodes = state.nodes;
        let errors = state.errors;

        ParseResult {
            root,
            nodes,
            errors,
        }
    }
}

//#[derive(Debug)]
pub struct ParseResult {
    pub root: NodeId,
    pub nodes: Arena,
    pub errors: Diagnostics<NodeId>,
}

impl ParseResult {
    pub fn display<'s, 'n>(&'n self, str: &'s str) -> DisplayParseResult<'s, 'n> {
        DisplayParseResult { str, result: self }
    }
}

pub struct DisplayParseResult<'s, 'n> {
    str: &'s str,
    result: &'n ParseResult,
}

impl<'s, 'n> fmt::Display for DisplayParseResult<'s, 'n> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arena = &self.result.nodes;
        let node = arena.get(self.result.root).display(self.str, arena);
        node.fmt(f)?;
        if self.result.errors.is_empty() {
            writeln!(f, "\n\n### No Errors ###")?;
        } else {
            writeln!(f, "\n\n### Errors ###")?;
        }

        for (node_id, error) in self.result.errors.iter() {
            writeln!(f, "{} @ {:?}", error.to_report(self.str), node_id)?;
        }
        Ok(())
    }
}
