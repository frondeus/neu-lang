use crate::HashCount;
use derive_more::Display;
use microtree_parser::TokenKind;
use logos::Logos;

#[derive(Debug, PartialEq, Clone, Copy, Display, Logos)]
#[logos(extras = HashCount)]
pub enum Token {
    #[display(fmt = "`\"`")]
    #[regex(r#"#*""#)]
    Close,

    #[error]
    #[display(fmt = "text")]
    Text,

}

pub type Lexer<'s, T = Token> = microtree_parser::Lexer<'s, T>;

impl<'s> TokenKind<'s> for Token {
    fn mergeable(self, other: Self) -> bool {
        match (self, other) {
            (Self::Text, Self::Text) => true,
            _ => false,
        }
    }
}
