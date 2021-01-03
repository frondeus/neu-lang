#![allow(dead_code)]
use super::*;
use microtree::{Ast, Red, TokenBuilder};
use smol_str::SmolStr;

#[allow(non_upper_case_globals)]
impl Nodes {
    pub const Atom: Name = Name::new("atom");
}

#[derive(Debug)]
pub struct Atom(Red);
impl Ast for Atom {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::Atom) {
            return None;
        }
        node.green().as_token()?;
        Some(Self(node))
    }

    fn red(&self) -> Red {
        self.0.clone()
    }
}

impl Atom {
    pub fn build(value: impl Into<SmolStr>) -> TokenBuilder<Atom> {
        TokenBuilder::custom(Nodes::Atom, value)
    }
}

impl IntoBuilder<Value> for TokenBuilder<Atom> {
    fn into_builder(self) -> AliasBuilder<Self, Value> {
        AliasBuilder::new(Nodes::Value, self)
    }
}
