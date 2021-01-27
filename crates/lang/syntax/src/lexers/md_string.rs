use crate::HashCount;
use microtree_parser::{TextSize, Source, TokenKind, CallbackResult};

fn lex_dquote(_bumped: TextSize, source: &mut Source<'_>, extras: &mut HashCount) -> bool {
    if extras.count > 0 {
        let hash_count =extras.count;
        let hash = "#".repeat(hash_count);
        if source.as_ref().starts_with(&hash) {
            source.bump(hash_count);
            true
        }
        else {
            false
        }
    } else {
        true
    }
}


#[derive(Debug, PartialEq, Clone, Copy, TokenKind)]
#[token_kind(extras = "HashCount", mergeable = "mergeable")]
pub enum Token {
    #[token_kind(token = r#"""#, callback = "lex_dquote")]
    Close,

    #[token_kind(error, display = "text")]
    Text,
}

pub type Lexer<'s, T = Token> = microtree_parser::Lexer<'s, T>;

fn mergeable(first: Token, other: Token) -> bool {
    match (first, other) {
        (Token::Text, Token::Text) => true,
        _ => false,
    }
}
