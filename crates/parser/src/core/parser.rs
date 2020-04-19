use crate::core::{Context, Node, State, Lexer};

pub trait Parser<Lex: Lexer> {
    fn parse(&self, state: &mut State<Lex>, ctx: &Context<Lex>) -> Node;
}

impl<Fun, Lex> Parser<Lex> for Fun
where
    Lex: Lexer,
    Fun: Fn(&mut State<Lex>, &Context<Lex>) -> Node,
{
    fn parse(&self, state: &mut State<Lex>, ctx: &Context<Lex>) -> Node {
        self(state, ctx)
    }
}
