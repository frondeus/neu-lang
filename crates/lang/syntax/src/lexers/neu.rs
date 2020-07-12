use crate::HashCount;
use derive_more::Display;
use neu_parser::{TextRange, TokenKind};

#[derive(Debug, PartialEq, Clone, Copy, Display)]
pub enum Token {
    #[display(fmt = "error")]
    Error,

    #[display(fmt = "` `, `\t`")]
    Whitespace,

    #[display(fmt = "`\n`, `\r\n`")]
    LineEnd,

    #[display(fmt = "comment")]
    Comment,

    #[display(fmt = "number")]
    Number,

    #[display(fmt = "`true`")]
    True,

    #[display(fmt = "`false`")]
    False,

    #[display(fmt = "`-`")]
    OpMinus,

    #[display(fmt = "`!`")]
    OpBang,

    #[display(fmt = "`+`")]
    OpPlus,

    #[display(fmt = "`*`")]
    OpStar,

    #[display(fmt = "`/`")]
    OpSlash,

    #[display(fmt = "`==`")]
    OpDEqual,

    #[display(fmt = "`=`")]
    OpAssign,

    #[display(fmt = "identifier")]
    Identifier,

    #[display(fmt = "`(`")]
    OpenP,

    #[display(fmt = "`)`")]
    CloseP,

    #[display(fmt = "`{{`")]
    OpenC,

    #[display(fmt = "`\"`")]
    DoubleQuote,

    #[display(fmt = "`md`")]
    MdQuote,

    #[display(fmt = "`}}`")]
    CloseC,

    #[display(fmt = "`[`")]
    OpenB,

    #[display(fmt = "`]`")]
    CloseB,

    #[display(fmt = "`,`")]
    Comma,

    #[display(fmt = "`.`")]
    OpDot,
}

pub type Lexer<T = Token> = neu_parser::Lexer<T>;

impl TokenKind for Token {
    type Extra = HashCount;

    fn is_mergeable(self, other: Self) -> bool {
        match (self, other) {
            (Self::Error, Self::Error) => true,
            (Self::LineEnd, Self::LineEnd) => true,
            _ => false,
        }
    }

    fn lex(lexer: &mut Lexer<Self>) -> Option<(Self, TextRange)> {
        let hash = lexer.extra.count;
        let input = lexer.input_mut();
        let i = input.as_ref();
        let peeked = i.chars().next()?;

        if i.starts_with("\r\n") {
            return Some((Token::LineEnd, input.chomp(2)));
        }
        if i.starts_with('\n') {
            return Some((Token::LineEnd, input.chomp(1)));
        }

        let is_whitespace = |c: &char| *c == ' ' || *c == '\t';

        if is_whitespace(&peeked) {
            let rest = i.chars().take_while(is_whitespace).count();

            return Some((Token::Whitespace, input.chomp(rest)));
        }
        if peeked.is_ascii_digit() {
            let rest = i.chars().take_while(|c| c.is_ascii_digit()).count();

            return Some((Token::Number, input.chomp(rest)));
        }

        if i.starts_with("/*") {
            let mut peeked = peeked;
            let mut i = &i[2..];
            let mut rest = 2;
            while !i.starts_with("*/") {
                i = &i[peeked.len_utf8()..];
                rest += 1;
                peeked = match i.chars().next() {
                    Some(p) => p,
                    None => return Some((Token::Error, input.chomp(2))),
                }
            }
            rest += 2;
            return Some((Token::Comment, input.chomp(rest)));
        }

        if i.starts_with("//") {
            let rest = i.chars().take_while(|c| *c != '\n').count();
            return Some((Token::Comment, input.chomp(rest)));
        }

        if i.starts_with("==") {
            return Some((Token::OpDEqual, input.chomp(2)));
        }

        if i.starts_with("true") {
            return Some((Token::True, input.chomp(4)));
        }

        if i.starts_with("false") {
            return Some((Token::False, input.chomp(5)));
        }

        if i.starts_with("md") {
            let mut rest = 2;
            let mut hash = 0;
            loop {
                let i = &i[rest..];
                if i.starts_with('"') {
                    let range = input.chomp(rest + 1);
                    lexer.extra.count = hash;
                    return Some((Token::MdQuote, range));
                }
                if i.starts_with('#') {
                    rest += 1;
                    hash += 1;
                    continue;
                }
                break;
            }
        }

        if peeked == '-' {
            return Some((Token::OpMinus, input.chomp(1)));
        }
        if peeked == '!' {
            return Some((Token::OpBang, input.chomp(1)));
        }
        if peeked == '+' && !i.starts_with("+++") {
            // TODO: Dirty hack
            return Some((Token::OpPlus, input.chomp(1)));
        }
        if peeked == '*' {
            return Some((Token::OpStar, input.chomp(1)));
        }
        if peeked == '/' {
            return Some((Token::OpSlash, input.chomp(1)));
        }
        if peeked == '=' {
            return Some((Token::OpAssign, input.chomp(1)));
        }
        if peeked == '.' {
            return Some((Token::OpDot, input.chomp(1)));
        }

        if peeked == ',' {
            return Some((Token::Comma, input.chomp(1)));
        }

        if peeked == '(' {
            return Some((Token::OpenP, input.chomp(1)));
        }
        if peeked == ')' {
            return Some((Token::CloseP, input.chomp(1)));
        }
        if peeked == '{' {
            return Some((Token::OpenC, input.chomp(1)));
        }
        if peeked == '}' {
            return Some((Token::CloseC, input.chomp(1)));
        }
        if peeked == '[' {
            return Some((Token::OpenB, input.chomp(1)));
        }
        if peeked == ']' {
            return Some((Token::CloseB, input.chomp(1)));
        }

        if hash > 0 {
            let pat = format!("{:#<width$}", "\"", width = hash + 1);
            if i.starts_with(&pat) {
                let range = input.chomp(pat.len());
                lexer.extra.count = 0;
                return Some((Token::DoubleQuote, range));
            }
        }

        if peeked == '"' {
            return Some((Token::DoubleQuote, input.chomp(1)));
        }

        if peeked.is_ascii_alphabetic() {
            let rest = i
                .chars()
                .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
                .count();
            return Some((Token::Identifier, input.chomp(rest)));
        }

        Some((Token::Error, input.chomp(1)))
    }
}
