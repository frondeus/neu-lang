use crate::{Context, Error as ParseError, Lexer, Node, Parser, TokenKind};
use std::borrow::Borrow;
use std::fmt;
use neu_diagnostic::Error;
use neu_diagnostic::NodeId as DiagnosticId;

#[derive(Clone, Copy)]
pub struct NodeId(pub(crate) usize);

impl fmt::Debug for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "N{}", self.0)
    }
}

impl From<DiagnosticId> for NodeId {
    fn from(id: DiagnosticId) -> Self {
        Self(id.id())
    }
}

impl Into<DiagnosticId> for NodeId {
    fn into(self) -> DiagnosticId {
        DiagnosticId::new(self.0)
    }
}


pub struct Ancestors<'a> {
    current: Option<NodeId>,
    arena: &'a Arena,
}

impl<'a> Iterator for Ancestors<'a> {
    type Item = NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current.map(|id| self.arena.get(id))?;

        let ancestor = current.parent();

        let current = self.current.take();
        self.current = ancestor;
        current
    }
}

#[derive(Default)]
pub struct Arena {
    nodes: Vec<Node>,
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
        let id = NodeId(len);
        for child_id in node.children.iter() {
            let child = self.get_mut(child_id);
            child.parent = Some(id);
        }
        self.nodes.push(node);
        id
    }

    pub fn ancestors(&self, id: NodeId) -> Ancestors {
        Ancestors {
            current: Some(id),
            arena: self,
        }
    }

    pub fn get(&self, id: impl Borrow<NodeId>) -> &Node {
        let id = *id.borrow();
        &self.nodes[id.0]
    }

    pub fn get_mut(&mut self, id: impl Borrow<NodeId>) -> &mut Node {
        let id = *id.borrow();
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
    errors: Vec<Error>,
    new_errors: Vec<ParseError<Tok>>,
    nodes: Arena,
}

impl<Tok: TokenKind + 'static> State<Tok> {
    fn new(lexer: Lexer<Tok>) -> Self {
        Self {
            lexer,
            errors: Default::default(),
            new_errors: Default::default(),
            nodes: Default::default(),
        }
    }

    pub(crate) fn transform<Tok2>(&mut self) -> State<Tok2>
    where
        Tok2: TokenKind,
        Tok::Extra: Into<Tok2::Extra>,
    {
        let lexer: Lexer<Tok2> = self.lexer.transform();
        State {
            lexer,
            nodes: self.nodes.take(),
            errors: self.errors.drain(..).collect(),
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
        self.nodes = other.nodes.take();
        self.errors = other.errors.drain(..).collect();
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

    pub fn error(&mut self, e: ParseError<Tok>) {
        self.new_errors.push(e);
    }

    pub fn commit_errors(&mut self, id: NodeId) {
        let new_errors = self.new_errors.drain(..).collect::<Vec<_>>();
        let whole_str = self.lexer().input().todo_whole_str();
        let mut new_errors = new_errors.into_iter().map(|e| {
                e.into_error(id, whole_str)
        }).collect::<Vec<_>>();
        self.errors
            .append(&mut new_errors);
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

#[derive(Debug)]
pub struct ParseResult {
    pub root: NodeId,
    pub nodes: Arena,
    pub errors: Vec<Error>,
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

        for error in self.result.errors.iter() {
            writeln!(f, "{} @ {:?}", error.desc(), error.location())?;
            //writeln!(f, "{} @ {:?}", error.display(self.str), node_id)?;
        }
        Ok(())
    }
}
