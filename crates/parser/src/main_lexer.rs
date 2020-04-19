use crate::core::{Input, TextRange, LexerState, Lexer};
use crate::Token;

pub struct MainLexer(LexerState<Token>);

impl MainLexer {
    pub fn new(i: &str) -> Self {
        Self(LexerState::new(i))
    }
}

impl Lexer for MainLexer {
    type Token = Token;
    fn build(state: LexerState<Token>) -> Self { Self(state) }
    fn state_mut(&mut self) -> &mut LexerState<Token> { &mut self.0 }
    fn state(&self) -> &LexerState<Token> { &self.0 }

    fn lex(input: &mut Input) -> Option<(Token, TextRange)> {
        let i = input.as_ref();
        let peeked = i.chars().next()?;
        if peeked.is_whitespace() {
            let rest = i.chars().take_while(|c| c.is_whitespace()).count();

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
                    None => return Some((Token::Error, input.chomp(2)))
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

        if peeked == '-' { return Some((Token::OpMinus, input.chomp(1))); }
        if peeked == '!' { return Some((Token::OpBang, input.chomp(1))); }
        if peeked == '+' { return Some((Token::OpPlus, input.chomp(1))); }
        if peeked == '*' { return Some((Token::OpStar, input.chomp(1))); }
        if peeked == '/' { return Some((Token::OpSlash, input.chomp(1))); }
        if peeked == '=' { return Some((Token::OpAssign, input.chomp(1))); }
        if peeked == '.' { return Some((Token::OpDot, input.chomp(1))); }

        if peeked == ',' { return Some((Token::Comma, input.chomp(1))); }

        if peeked == '(' { return Some((Token::OpenP, input.chomp(1))); }
        if peeked == ')' { return Some((Token::CloseP, input.chomp(1))); }
        if peeked == '{' { return Some((Token::OpenC, input.chomp(1))); }
        if peeked == '}' { return Some((Token::CloseC, input.chomp(1))); }
        if peeked == '[' { return Some((Token::OpenB, input.chomp(1))); }
        if peeked == ']' { return Some((Token::CloseB, input.chomp(1))); }

        if peeked == '"' { return Some((Token::OpenS, input.chomp(1))); }

        if peeked.is_ascii_alphabetic() {
            let rest = i.chars()
                .take_while(|c| c.is_ascii_alphanumeric() || *c == '_').count();
            return Some((Token::Identifier, input.chomp(rest)));
        }

        Some((Token::Error, input.chomp(1)))
    }
}
