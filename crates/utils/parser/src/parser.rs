use crate::{Context, Node, State, TokenKind};

pub trait Parser<Tok: TokenKind> {
    fn parse(&self, state: &mut State<Tok>, ctx: &Context<Tok>) -> Node;
}

impl<Fun, Tok> Parser<Tok> for Fun
where
    Tok: TokenKind,
    Fun: Fn(&mut State<Tok>, &Context<Tok>) -> Node,
{
    fn parse(&self, state: &mut State<Tok>, ctx: &Context<Tok>) -> Node {
        self(state, ctx)
    }
}
