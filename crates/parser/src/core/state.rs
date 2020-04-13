use std::fmt;
use crate::core::{Context, Error, Lexer, Node, Parser};
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

pub struct State {
    lexer: Lexer,
    errors: Vec<(NodeId, Error)>,
    new_errors: Vec<Error>,
    nodes: Arena
}

impl State {
    fn new(lexer: Lexer) -> Self {
        Self {
            lexer,
            errors: Default::default(),
            new_errors: Default::default(),
            nodes: Default::default(),
        }
    }

    pub fn nodes(&mut self) -> &mut Arena {
        &mut self.nodes
    }

    pub fn lexer(&self) -> &Lexer {
        &self.lexer
    }

    pub fn lexer_mut(&mut self) -> &mut Lexer {
        &mut self.lexer
    }

    pub fn error(&mut self, e: Error) {
        self.new_errors.push(e);
    }

    pub fn commit_errors(&mut self, id: NodeId) {
        self.errors.extend(self.new_errors.drain(..).map(|e| (id, e)));
    }

    pub fn parse(lexer: Lexer, parser: impl Parser) -> ParseResult {
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
pub struct ParseResult {
    pub root: NodeId,
    pub nodes: Arena,
    pub errors: Vec<(NodeId, Error)>,
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

impl <'s, 'n> fmt::Display for DisplayParseResult<'s, 'n> {
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
