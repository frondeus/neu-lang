use std::fmt;
use crate::core::{Context, Error, Lexer, Node, Parser, TokenKind};
use std::borrow::Borrow;

#[derive(Clone, Copy)]
pub struct NodeId(pub(crate) usize);

impl fmt::Debug for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "N{}", self.0)
    }
}

#[derive(Default)]
pub struct Arena {
    nodes: Vec<Node>
}

impl fmt::Debug for Arena {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Arena")?;
        for (i, n) in self.nodes.iter().enumerate() {
            write!(f, "\tN{}: ", i)?;
            writeln!(f, "{:?}", n)?;
        }
        Ok(())
    }
}

impl Arena {
    pub fn take(&mut self) -> Self {
        let mut nodes = vec![];
        nodes.append(&mut self.nodes);
        Self { nodes }
    }

    pub fn add(&mut self, node: Node) -> NodeId {
        let len = self.nodes.len();
        self.nodes.push(node);
        NodeId(len)
    }

    pub fn get(&self, id: impl Borrow<NodeId>) -> &Node {
        let id = *id.borrow();
        &self.nodes[id.0]
    }

    pub fn get_mut(&mut self, id: NodeId) -> &mut Node {
        &mut self.nodes[id.0]
    }

    pub fn iter(&self) -> impl Iterator<Item = &Node> {
        self.nodes.iter()
    }

    pub fn enumerate(&self) -> impl Iterator<Item = (NodeId, &Node)> {
        self.nodes.iter().enumerate().map(|(id, n)| (NodeId(id), n))
    }
}

pub struct State<Lex: Lexer> {
    lexer: Lex,
    errors: Vec<(NodeId, Error<Lex::Token>)>,
    new_errors: Vec<Error<Lex::Token>>,
    nodes: Arena
}

impl<Lex: Lexer> State<Lex> {
    fn new(lexer: Lex) -> Self {
        Self {
            lexer,
            errors: Default::default(),
            new_errors: Default::default(),
            nodes: Default::default(),
        }
    }

    pub(crate) fn transform<Lex2: Lexer>(&mut self) -> State<Lex2> {
        let lexer: Lex2 = self.lexer.transform();
        State {
            lexer,
            nodes: self.nodes.take(),
            errors: Default::default(),
            new_errors: Default::default(),
        }
    }

    pub(crate) fn restore<Lex2: Lexer>(&mut self, mut other: State<Lex2>) {
        let lexer: Lex = other.lexer().transform();
        self.lexer = lexer;
        self.nodes = other.nodes.take();
    }

    pub fn nodes(&mut self) -> &mut Arena {
        &mut self.nodes
    }

    pub fn lexer(&self) -> &Lex {
        &self.lexer
    }

    pub fn lexer_mut(&mut self) -> &mut Lex {
        &mut self.lexer
    }

    pub fn error(&mut self, e: Error<Lex::Token>) {
        self.new_errors.push(e);
    }

    pub fn commit_errors(&mut self, id: NodeId) {
        self.errors.extend(self.new_errors.drain(..).map(|e| (id, e)));
    }

    pub fn parse(lexer: Lex, parser: impl Parser<Lex>) -> ParseResult<Lex::Token> {
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

#[derive(Debug)]
pub struct ParseResult<Tok: TokenKind> {
    pub root: NodeId,
    pub nodes: Arena,
    pub errors: Vec<(NodeId, Error<Tok>)>,
}

impl<Tok: TokenKind> ParseResult<Tok> {
    pub fn display<'s, 'n>(&'n self, str: &'s str) -> DisplayParseResult<'s, 'n, Tok> {
        DisplayParseResult { str, result: self }
    }
}

pub struct DisplayParseResult<'s, 'n, Tok: TokenKind> {
    str: &'s str,
    result: &'n ParseResult<Tok>,
}

impl <'s, 'n, Tok: TokenKind> fmt::Display for DisplayParseResult<'s, 'n, Tok> {
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
            writeln!(f, "{} @ {:?}", error.display(self.str), node_id)?;
        }
        Ok(())
    }
}
