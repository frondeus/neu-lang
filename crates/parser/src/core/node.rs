use crate::core::{Lexer, NodeBuilder, TextRange, NodeId, Arena};
use std::collections::BTreeSet;
use std::fmt;
use itertools::Itertools;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Name(pub(crate) &'static str);
impl Name {
    pub const fn new(name: &'static str) -> Self {
        Self(name)
    }
}

impl fmt::Debug for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
pub struct Node {
    pub span: TextRange,
    pub names: BTreeSet<Name>,
    pub children: Vec<NodeId>,
}

impl Node {
    pub fn builder(lexer: &Lexer) -> NodeBuilder {
        NodeBuilder::new(lexer)
    }

    pub fn is(&self, name: Name) -> bool {
        self.names.contains(&name)
    }

    pub fn with_name(mut self, name: Name) -> Node {
        self.names.insert(name);
        self
    }

    pub fn display<'s, 'n, 'a>(&'n self, str: &'s str, arena: &'a Arena) -> DisplayNode<'s, 'n, 'a> {
        DisplayNode {
            str,
            node: self,
            arena
        }
    }

}

pub struct DisplayNode<'s, 'n, 'a> {
    str: &'s str,
    node: &'n Node,
    arena: &'a Arena
}

impl <'s, 'n, 'a> fmt::Display for DisplayNode<'s, 'n, 'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width = f.width().unwrap_or_default();
        if width > 0 { write!(f, "{:width$}", " ", width = width)?; }

        let span = &self.str[self.node.span];
        writeln!(f, "{} @ {:?} = `{}`", self.node.names.iter().join(", ").to_uppercase(), self.node.span, span)?;
        let c_width = width + 4;
        for child in self.node.children.iter().map(|child| self.arena.get(child).display(self.str, self.arena)) {
            write!(f, "{:width$}", child, width = c_width)?;
        }
        Ok(())
    }
}