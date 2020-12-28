use crate::HashCount;
use derive_more::Display;
use microtree_parser::TokenKind;
use logos::Logos;

#[derive(Debug, PartialEq, Clone, Copy, Display, Logos)]
#[logos(extras = HashCount)]
pub enum Token {
    #[display(fmt = "text")]
    #[error]
    Text,

    #[display(fmt = "`${{`")]
    #[token("${")]
    OpenI,

    #[display(fmt = "`}}`")]
    #[token("}")]
    CloseI,

    #[display(fmt = "`\"`")]
    #[token("\"")]
    Close,
}

pub type Lexer<'s> = microtree_parser::Lexer<'s, Token>;

impl<'s> TokenKind<'s> for Token {
    fn mergeable(self, other: Self) -> bool {
        match (self, other) {
            (Self::Text, Self::Text) => true,
            (Self::Text, Self::CloseI) => true,
            _ => false,
        }
    }
}
