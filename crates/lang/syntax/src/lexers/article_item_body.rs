use derive_more::Display;
use neu_parser::{TextRange, TokenKind};

#[derive(Debug, PartialEq, Clone, Copy, Display)]
pub enum Token {
    #[display("text")]
    Text,
}

pub type Lexer<T = Token> = neu_parser::Lexer<T>;

impl TokenKind for Token {
    type Extra = ();

    fn is_mergeable(self, other: Self) -> bool {
        match (self, other) {
            (Self::Text, Self::Text) => true,
            //_ => false,
        }
    }

    fn lex(lexer: &mut Lexer<Self>) -> Option<(Self, TextRange)> {
        let input = lexer.input_mut();

        let i = input.as_ref();
        if i.is_empty() {
            return None;
        }

        //if i.starts_with("+++") {
        //return Some((Token::ThreePlus, input.chomp(3)));
        //}

        Some((Token::Text, input.chomp(1)))
    }
}
