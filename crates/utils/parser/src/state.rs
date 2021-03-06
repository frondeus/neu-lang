use crate::{Arena, Context, Lexer, NodeId, Parser, TokenKind};
use neu_diagnostics::{Diagnostic, DiagnosticVec, Diagnostics};
use std::fmt;

pub struct State<Tok: TokenKind> {
    lexer: Lexer<Tok>,
    new_errors: DiagnosticVec,
    arena: Arena,
    errors: Diagnostics<NodeId>
}

impl<Tok: TokenKind> State<Tok> {
    fn new(lexer: Lexer<Tok>) -> Self {
        Self {
            lexer,
            new_errors: Default::default(),
            arena: Default::default(),
            errors: Default::default()
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
            arena: std::mem::take(&mut self.arena),
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
        self.arena = std::mem::take(&mut other.arena);
        self.errors = std::mem::take(&mut other.errors);
    }

    pub fn nodes(&mut self) -> &mut Arena {
        &mut self.arena
    }

    pub fn lexer(&self) -> &Lexer<Tok> {
        &self.lexer
    }

    pub fn lexer_mut(&mut self) -> &mut Lexer<Tok> {
        &mut self.lexer
    }

    pub fn error(&mut self, e: Diagnostic) {
        self.new_errors.push(e);
    }

    pub fn commit_errors(&mut self, id: NodeId) {
        for error in self.new_errors.drain(..) {
            self.errors.add(id, error);
        }
    }

    pub fn parse(lexer: Lexer<Tok>, parser: impl Parser<Tok>) -> ParseResult {
        let mut state = Self::new(lexer);
        let ctx = Context::default();
        let root = parser.parse(&mut state, &ctx);
        let root = state.nodes().add(root);
        let arena = state.arena;
        let errors = state.errors;

        ParseResult { root, arena, errors }
    }
}

//#[derive(PartialEq, Eq, Clone)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseResult {
    pub root: NodeId,
    pub arena: Arena,
    pub errors: Diagnostics<NodeId>
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
        let arena = &self.result.arena;
        let node = arena.get(self.result.root).display(self.str, arena);
        node.fmt(f)?;
        Ok(())
    }
}
