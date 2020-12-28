use derive_more::Display;
use microtree_parser::TokenKind;
use logos::Logos;

#[derive(Debug, PartialEq, Clone, Copy, Display, Logos)]
pub enum Token {
    #[display(fmt = "`+++`")]
    #[token("+++")]
    ThreePlus,

    #[display(fmt = "error")]
    #[error]
    Error,
}

pub type Lexer<'s, T = Token> = microtree_parser::Lexer<'s, T>;

impl<'s> TokenKind<'s> for Token {
    fn mergeable(self, other: Self) -> bool {
        match (self, other) {
            (Self::Error, Self::Error) => true,
            _ => false,
        }
    }
}
