use microtree_parser::{Source, TokenKind};

#[derive(Debug, PartialEq, Clone, Copy, TokenKind)]
#[token_kind(mergeable = "mergeable")]
pub enum Token {
    #[token_kind(token = "+++")]
    ThreePlus,

    #[token_kind(token = "++")]
    PlusPlus,

    #[token_kind(regex = r"[ \t]+", display = r"` `, `\t`")]
    InlineWhitespace,

    #[token_kind(regex = r"[\r?\n]+", display = r"`\n`, `\r\n`")]
    NewLine,

    #[token_kind(regex = r"[0-9a-fA-F]{8}", display = "item id")]
    ItemId,

    #[token_kind(regex = r"[A-Za-z_]+[A-Za-z_0-9]*", display="identifier")]
    Identifier,

    #[token_kind(token = ":")]
    Colon,

    #[token_kind(error)]
    Error,
}

pub type Lexer<'s, T = Token> = microtree_parser::Lexer<'s, T>;

fn mergeable(first: Token, other: Token) -> bool {
    match (first, other) {
        (Token::Error, Token::Error) => true,
        _ => false,
    }
}
