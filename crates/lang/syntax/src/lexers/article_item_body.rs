use derive_more::Display;
use microtree_parser::TokenKind;
use logos::Logos;

#[derive(Display, Debug, PartialEq, Clone, Copy, Logos)]
pub enum Token {
    #[display(fmt = "text")]
    #[error]
    Text,

    #[display(fmt = "`++ end ++`")]
    #[token("++ end ++")]
    PlusPlusEnd,

    #[display(fmt = "`++`")]
    #[token("++")]
    PlusPlus,

    #[display(fmt = "`[+`")]
    #[token("[+")]
    OpenBl,

    #[display(fmt = "`+]`")]
    #[token("+]")]
    CloseBl,
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
