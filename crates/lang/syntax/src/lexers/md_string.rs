use crate::HashCount;
use derive_more::Display;
use microtree_parser::TokenKind;
use logos::Logos;

#[derive(Debug, PartialEq, Clone, Copy, Display, Logos)]
#[logos(extras = HashCount)]
pub enum Token {
    #[display(fmt = "`\"`")]
    #[token(r#"""#, |lex| {
        if lex.extras.count > 0 {
            let hash_count = lex.extras.count;
            let hash = "#".repeat(hash_count);
            if lex.remainder().starts_with(&hash) {
            //panic!("remainder: `{}`, hash_count: {}", &remainder, hash_count);
                lex.bump(hash_count);
                // We don't reset extras cause its done on Neu side
                //lex.extras.count = 0;
                true
            }
            else {
                false
            }
        } else {
            true
        }
    })]
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
