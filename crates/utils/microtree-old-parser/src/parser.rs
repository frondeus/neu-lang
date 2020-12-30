use crate::{Context, Lexer, State, TokenKind};
use microtree::Green;

pub trait Parser<'source, Tok: TokenKind<'source>> {
    fn parse<'ctx>(&mut self, state: State<'source, Tok>,
             context: &Context<'source, 'ctx, Tok>) -> (Option<Green>, State<'source, Tok>);
}

pub trait Trivia<'source, Tok: TokenKind<'source>> {
    fn parse(&self, lexer: &mut Lexer<'source, Tok>);
}

impl<'source, Fun, Tok> Trivia<'source, Tok> for Fun
where
    Tok: TokenKind<'source>,
    Fun: Fn(&mut Lexer<'source, Tok>),
{
    fn parse(&self, lexer: &mut Lexer<'source, Tok>) {
        self(lexer)
    }
}
