use derive_more::Display;
use neu_parser::{TextRange, TokenKind};

#[derive(Display, Debug, PartialEq, Clone, Copy)]
pub enum Token {
    #[display(fmt = "text")]
    Text,

    #[display(fmt = "`++ end ++`")]
    PlusPlusEnd,

    #[display(fmt = "`++`")]
    PlusPlus,

    #[display(fmt = "`[+`")]
    OpenBl,

    #[display(fmt = "`+]`")]
    CloseBl,
}

pub type Lexer<T = Token> = neu_parser::Lexer<T>;

impl TokenKind for Token {
    type Extra = ();

    fn is_mergeable(self, other: Self) -> bool {
        match (self, other) {
            (Self::Text, Self::Text) => true,
            _ => false,
        }
    }

    fn lex(lexer: &mut Lexer<Self>) -> Option<(Self, TextRange)> {
        let input = lexer.input_mut();

        let i = input.as_ref();
        if i.is_empty() {
            return None;
        }

        if i.starts_with("++ end ++") {
            return Some((Token::PlusPlusEnd, input.chomp(9)));
        }

        if i.starts_with("++") {
            return Some((Token::PlusPlus, input.chomp(2)));
        }

        if i.starts_with("[+") {
            return Some((Token::OpenBl, input.chomp(2)));
        }

        if i.starts_with("+]") {
            return Some((Token::CloseBl, input.chomp(2)));
        }

        Some((Token::Text, input.chomp(1)))
    }
}
