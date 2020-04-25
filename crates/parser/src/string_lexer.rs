use crate::core::{Lexer, TextRange, TokenKind};
use crate::{StrToken, HashCount};

pub type StringLexer = Lexer<StrToken>;

impl TokenKind for StrToken {
    type Extra = HashCount;

    fn is_mergeable(self, other: Self) -> bool {
        match (self, other) {
            (Self::Text, Self::Text) => true,
            (Self::Text, Self::CloseI) => true,
            _ => false
        }
    }

    fn lex(lexer: &mut Lexer<Self>) -> Option<(Self, TextRange)> {
        let input = lexer.input_mut();
        let i = input.as_ref();
        if i.is_empty() { return None; }
        if i.starts_with('"') {
            return Some((StrToken::Close, input.chomp(1)));
        }

        if i.starts_with("${") {
            return Some((StrToken::OpenI, input.chomp(2)));
        }

        if i.starts_with('}') {
            return Some((StrToken::CloseI, input.chomp(1)));
        }

        Some((StrToken::Text, input.chomp(1)))
    }
}
