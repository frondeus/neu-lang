#![allow(dead_code)]
use super::*;
use microtree::{Ast, Red, TokenBuilder};

#[allow(non_upper_case_globals)]
impl Nodes {
    pub const Number: Name = Name::new("number");
}

#[derive(Debug)]
pub struct Number(Red);
impl Ast for Number {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::Number) {
            return None;
        }
        node.green().as_token()?;
        Some(Self(node))
    }

    fn red(&self) -> Red {
        self.0.clone()
    }
}

impl Number {
    pub fn build(value: u32) -> TokenBuilder<Number> {
        TokenBuilder::custom(Nodes::Number, value.to_string())
    }
}

impl IntoBuilder<Value> for TokenBuilder<Number> {
    fn into_builder(self) -> AliasBuilder<Self, Value> {
        AliasBuilder::new(Nodes::Value, self)
    }
}
