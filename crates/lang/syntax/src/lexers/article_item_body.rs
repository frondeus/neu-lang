use microtree_parser::{TokenKind, Source};

#[derive(Debug, PartialEq, Clone, Copy, TokenKind)]
#[token_kind(mergeable = "mergeable")]
pub enum Token {
    #[token_kind(token = "++ end ++")]
    PlusPlusEnd,

    #[token_kind(token = "++")]
    PlusPlus,

    #[token_kind(token = "[+")]
    OpenBl,

    #[token_kind(token = "+]")]
    CloseBl,

    #[token_kind(error, display = "text")]
    Text,
}

pub type Lexer<'s, T = Token> = microtree_parser::Lexer<'s, T>;

fn mergeable(first: Token, other: Token) -> bool {
    match (first, other) {
        (Token::Text, Token::Text) => true,
        (Token::Text, Token::CloseBl) => true,
        _ => false,
    }
}
