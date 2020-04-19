use crate::core::{LexerState, Lexer, Input, TextRange};
use crate::StrToken;

pub struct StringLexer(LexerState<StrToken>);

impl Lexer for StringLexer {
    type Token = StrToken;

    fn build(state: LexerState<StrToken>) -> Self { Self(state) }
    fn state_mut(&mut self) -> &mut LexerState<Self::Token> { &mut self.0 }
    fn state(&self) -> &LexerState<Self::Token> { &self.0 }

    fn lex(input: &mut Input) -> Option<(Self::Token, TextRange)> {
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
