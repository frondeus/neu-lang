use crate::HashCount;
use derive_more::Display;
use neu_parser::{Lexer, TextRange, TokenKind};

#[derive(Debug, PartialEq, Clone, Copy, Display)]
pub enum Token {
    #[display(fmt = "text")]
    Text,

    #[display(fmt = "`${{`")]
    OpenI,

    #[display(fmt = "`}}`")]
    CloseI,

    #[display(fmt = "`\"`")]
    Close,
}

pub type StringLexer = Lexer<Token>;

impl TokenKind for Token {
    type Extra = HashCount;

    fn is_mergeable(self, other: Self) -> bool {
        match (self, other) {
            (Self::Text, Self::Text) => true,
            (Self::Text, Self::CloseI) => true,
            _ => false,
        }
    }

    fn lex(lexer: &mut Lexer<Self>) -> Option<(Self, TextRange)> {
        let input = lexer.input_mut();
        let i = input.as_ref();
        if i.is_empty() {
            return None;
        }
        if i.starts_with('"') {
            return Some((Token::Close, input.chomp(1)));
        }

        if i.starts_with("${") {
            return Some((Token::OpenI, input.chomp(2)));
        }

        if i.starts_with('}') {
            return Some((Token::CloseI, input.chomp(1)));
        }

        Some((Token::Text, input.chomp(1)))
    }
}
