use std::fmt;
use crate::core::{Context, Error, TokenKind, Node, Parser, Lexer};
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

    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &Node> {
        self.nodes.iter()
    }

    pub fn enumerate(&self) -> impl Iterator<Item = (NodeId, &Node)> {
        self.nodes.iter().enumerate().map(|(id, n)| (NodeId(id), n))
    }
}

pub struct State<Tok: TokenKind> {
    lexer: Lexer<Tok>,
    errors: Vec<(NodeId, Error<Tok>)>,
    new_errors: Vec<Error<Tok>>,
    nodes: Arena
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

    pub(crate) fn transform<Tok2>(&mut self) -> State<Tok2>
        where Tok2: TokenKind,
              Tok::Extra: Into<Tok2::Extra>
    {
        let lexer: Lexer<Tok2> = self.lexer.transform();
        State {
            lexer,
            nodes: self.nodes.take(),
            errors: Default::default(),
            new_errors: Default::default(),
        }
    }

    pub(crate) fn restore<Tok2>(&mut self, mut other: State<Tok2>)
        where Tok2: TokenKind,
              Tok2::Extra: Into<Tok::Extra>
    {
        let lexer: Lexer<Tok> = other.lexer().transform();
        self.lexer = lexer;
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

    pub fn error(&mut self, e: Error<Tok>) {
        self.new_errors.push(e);
    }

    pub fn commit_errors(&mut self, id: NodeId) {
        self.errors.extend(self.new_errors.drain(..).map(|e| (id, e)));
    }

    pub fn parse(lexer: Lexer<Tok>, parser: impl Parser<Tok>) -> ParseResult<Tok> {
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
