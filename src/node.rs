use crate::lexer::Lexer;
use crate::token::TokenKind;
use crate::TextRange;
use std::collections::BTreeSet;
use crate::node_builder::NodeBuilder;
use crate::parser::Context;
use std::fmt;

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

pub struct Nodes;
#[allow(non_upper_case_globals)]
impl Nodes {
    pub const Virtual: Name = Name("Virtual");
    pub const Trivia: Name = Name("Trivia");
    pub const Token: Name = Name("Token");
    pub const Error: Name = Name("Error");
}

#[derive(Debug)]
pub struct Node {
    pub span: TextRange,
    pub names: BTreeSet<Name>,
    pub children: Vec<Node>,
}

impl Node {
    pub fn builder<K, Lex>(context: &Context<Lex>) -> NodeBuilder
    where
        K: TokenKind,
        Lex: Lexer<K>
    {
        NodeBuilder::new(context)
    }

    pub fn is(&self, name: Name) -> bool {
        self.names.contains(&name)
    }

    pub fn with_name(mut self, name: Name) -> Node {
        self.names.insert(name);
        self
    }
}

