#![allow(dead_code)]
use super::*;
use microtree::{Ast, Red, TokenBuilder};
use smol_str::SmolStr;

#[allow(non_upper_case_globals)]
impl Nodes {
    pub const Atom: Name = Name::new("atom");
    pub const Text: Name = Name::new("text");
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

#[derive(Debug)]
pub struct Text(Red);
impl Ast for Text {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::Text) {
            return None;
        }
        node.green().as_token()?;
        Some(Self(node))
    }

    fn red(&self) -> Red {
        self.0.clone()
    }
}

impl Text {
    pub fn build(value: impl Into<SmolStr>) -> TokenBuilder<Text> {
        TokenBuilder::custom(Nodes::Text, value)
    }
}

impl IntoBuilder<StrValue> for TokenBuilder<Text> {
    fn into_builder(self) -> AliasBuilder<Self, StrValue> {
        AliasBuilder::new(Nodes::StrValue, self)
    }
}
