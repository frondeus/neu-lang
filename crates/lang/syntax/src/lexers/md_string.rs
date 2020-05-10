use crate::HashCount;
use derive_more::Display;
use neu_parser::{TextRange, TokenKind};

#[derive(Debug, PartialEq, Clone, Copy, Display)]
pub enum Token {
    #[display(fmt = "text")]
    Text,

    #[display(fmt = "`\"`")]
    Close,
}

pub type Lexer<T = Token> = neu_parser::Lexer<T>;

impl TokenKind for Token {
    type Extra = HashCount;

    fn is_mergeable(self, other: Self) -> bool {
        match (self, other) {
            (Self::Text, Self::Text) => true,
            _ => false,
        }
    }

    fn lex(lexer: &mut Lexer<Self>) -> Option<(Self, TextRange)> {
        let hash = lexer.extra.count;
        let input = lexer.input_mut();
        let i = input.as_ref();
        if i.is_empty() {
            return None;
        }
        let pat = format!("{:#<width$}", "\"", width = hash + 1);
        if i.starts_with(&pat) {
            return Some((Token::Close, input.chomp(pat.len())));
        }

        Some((Token::Text, input.chomp(1)))
    }
}
