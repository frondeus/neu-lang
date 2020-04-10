use std::fmt;
use crate::core::{Context, Error, Lexer, Node, Parser};
use std::borrow::Borrow;

#[derive(Clone, Copy)]
pub struct NodeId(usize);

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
}

pub struct State {
    lexer: Lexer,
    errors: Vec<(NodeId, Error)>,
    nodes: Arena
}

impl State {
    fn new(lexer: Lexer) -> Self {
        Self {
            lexer,
            errors: Default::default(),
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

    pub fn error(&mut self, node_id: NodeId, e: Error) {
        self.errors.push((node_id, e));
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
        writeln!(f, "\n\n### Errors ###")?;
        write!(f, "{:#?}", self.result.errors)
        //write!(f, "{:#?}", self.result)
    }
}
