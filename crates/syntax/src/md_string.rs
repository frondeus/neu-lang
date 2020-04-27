use derive_more::Display;
use neu_parser::{Lexer, TextRange, TokenKind};
use crate::HashCount;

#[derive(Debug, PartialEq, Clone, Copy, Display)]
pub enum MdStrToken {
    #[display(fmt = "text")]
    Text,

    #[display(fmt = "`\"`")]
    Close,
}

pub type MdStringLexer = Lexer<MdStrToken>;

impl TokenKind for MdStrToken {
    type Extra = HashCount;

    fn is_mergeable(self, other: Self) -> bool {
        match (self, other) {
            (Self::Text, Self::Text) => true,
            _ => false
        }
    }

    fn lex(lexer: &mut Lexer<Self>) -> Option<(Self, TextRange)> {
        let hash = lexer.extra.count;
        let input = lexer.input_mut();
        let i = input.as_ref();
        if i.is_empty() { return None; }
        let pat = format!("{:#<width$}", "\"", width = hash + 1);
        if i.starts_with(&pat) {
            return Some((MdStrToken::Close, input.chomp(pat.len())));
        }

        Some((MdStrToken::Text, input.chomp(1)))
    }
}
