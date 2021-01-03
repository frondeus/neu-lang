use derive_more::Display;
use microtree_parser::TokenKind;
use logos::Logos;

#[derive(Display, Debug, PartialEq, Clone, Copy, Logos)]
pub enum Token {
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

    #[display(fmt = "text")]
    #[regex(".")]
    Text,

    #[display(fmt = "error")]
    #[error]
    Error,

}

pub type Lexer<'s, T = Token> = microtree_parser::Lexer<'s, T>;

impl<'s> TokenKind<'s> for Token {
    fn mergeable(self, other: Self) -> bool {
        match (self, other) {
            (Self::Text, Self::Text) |
            (Self::Error, Self::Text) |
            (Self::Text, Self::Error) |
            (Self::Error, Self::Error) => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexer_test() {
        let input = r#"\n\nAla ma kota"#;

        let lexer: Lexer<'_, Token> = Lexer::new(input);
        let tokens: Vec<_> = lexer.collect();
        dbg!(&tokens);
        todo!();
    }
}
