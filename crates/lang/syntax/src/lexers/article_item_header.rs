use derive_more::Display;
use neu_parser::{TextRange, TokenKind};

#[derive(Debug, PartialEq, Clone, Copy, Display)]
pub enum Token {
    #[display(fmt = "`+++`")]
    ThreePlus,

    #[display("` `, `\t`")]
    InlineWhitespace,

    #[display("`\n`, `\r\n`")]
    NewLine,

    #[display("identifier")]
    Identifier,

    #[display("`:`")]
    Colon,

    #[display("item id")]
    ItemId,

    #[display("error")]
    Error,
}

pub type Lexer<T = Token> = neu_parser::Lexer<T>;

impl TokenKind for Token {
    type Extra = ();

    fn is_mergeable(self, other: Self) -> bool {
        match (self, other) {
            (Self::Error, Self::Error) => true,
            _ => false,
        }
    }

    fn lex(lexer: &mut Lexer<Self>) -> Option<(Self, TextRange)> {
        let input = lexer.input_mut();
        let i = input.as_ref();
        let peeked = i.chars().next()?;

        let is_inline_ws = |c: &char| *c == ' ' || *c == '\t';

        if is_inline_ws(&peeked) {
            let rest = i.chars().take_while(is_inline_ws).count();
            return Some((Token::InlineWhitespace, input.chomp(rest)));
        }

        if i.starts_with("+++") {
            return Some((Token::ThreePlus, input.chomp(3)));
        }

        if i.starts_with('\n') {
            return Some((Token::NewLine, input.chomp(1)));
        }
        if i.starts_with("\r\n") {
            return Some((Token::NewLine, input.chomp(2)));
        }

        if peeked == ':' {
            return Some((Token::Colon, input.chomp(1)));
        }

        if i.chars().take(8).all(|c| c.is_ascii_hexdigit() || c == '_') {
            let rest = i.chars().take(8).count();

            if rest == 8 {
                return Some((Token::ItemId, input.chomp(rest)));
            }
        }

        if peeked.is_ascii_alphabetic() {
            let rest = i
                .chars()
                .take_while(|c| {
                    c.is_ascii_alphabetic() || *c == '_'
                })
                .count();

            return Some((Token::Identifier, input.chomp(rest)));
        }

        Some((Token::Error, input.chomp(1)))
    }
}
