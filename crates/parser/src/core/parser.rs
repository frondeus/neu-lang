use crate::core::{Context, Node, State};

pub trait Parser {
    fn parse(&self, state: &mut State, ctx: &Context) -> Node;
}

impl<Fun> Parser for Fun
where
    Fun: Fn(&mut State, &Context) -> Node,
{
    fn parse(&self, state: &mut State, ctx: &Context) -> Node {
        self(state, ctx)
    }
}
