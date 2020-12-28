use derive_more::Display;
use microtree_parser::TokenKind;
use logos::Logos;

#[derive(Debug, PartialEq, Clone, Copy, Display, Logos)]
pub enum Token {
    #[display(fmt = "`+++`")]
    #[token("+++")]
    ThreePlus,

    #[display(fmt = "`++`")]
    #[token("++")]
    PlusPlus,

    #[display(fmt = "` `, `\t`")]
    #[regex(r#"[ \t]+"#)]
    InlineWhitespace,

    #[display(fmt = "`\n`, `\r\n`")]
    #[regex(r#"[\r?\n]+"#)]
    NewLine,

    #[display(fmt = "identifier")]
    #[regex(r#"[A-Za-z_]+[A-Za-z_0-9]*"#)]
    Identifier,

    #[display(fmt = "`:`")]
    #[token(":")]
    Colon,

    #[display(fmt = "item id")]
    #[regex(r#"[0-9a-fA-F][0-9a-fA-F][0-9a-fA-F][0-9a-fA-F][0-9a-fA-F][0-9a-fA-F][0-9a-fA-F][0-9a-fA-F]"#)]
    ItemId,

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
