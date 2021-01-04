use microtree_parser::{TokenKind, Source};

#[derive(Debug, PartialEq, Clone, Copy, TokenKind)]
#[token_kind(mergeable = "mergeable")]
pub enum Token {
    #[token_kind(token = "+++")]
    ThreePlus,

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
