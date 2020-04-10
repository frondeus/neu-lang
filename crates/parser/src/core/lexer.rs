use crate::core::{Input, PeekableIterator, Spanned, TextRange};
use crate::{lex, Token};

pub struct Lexer {
    input: Input,
    #[allow(clippy::option_option)]
    peeked: Option<Option<Spanned<Token>>>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.into(),
            peeked: None,
        }
    }
}

impl Lexer {
    fn lex(&mut self) -> Option<Spanned<Token>> {
        if let Some(peeked) = self.peeked.take() {
            if let Some(peeked) = peeked.as_ref() {
                self.input.cursor = peeked.span.end();
            }
            return peeked;
        }

        let (token, span) = lex(&mut self.input)?;

        Some(Spanned::new(span, token))
    }
}

impl Iterator for Lexer {
    type Item = Spanned<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut first = self.lex()?;

        while let Token::Error = first.kind {
            match self.peek() {
                Some(token) if token.kind == Token::Error => {
                    first.span = TextRange::covering(first.span, token.span);
                    self.lex();
                }
                _ => break,
            }
        }
        Some(first)
    }
}

impl PeekableIterator for Lexer {
    fn peek(&mut self) -> Option<&Self::Item> {
        if self.peeked.is_none() {
            let i = self.input.cursor;
            self.peeked = Some(self.next());
            self.input.cursor = i;
        }

        self.peeked.as_ref().and_then(|i| i.as_ref())
    }
}

impl Lexer {
    pub fn input(&self) -> Input {
        self.input.clone()
    }

    pub fn set_input(&mut self, input: Input) {
        self.input = input;
    }
}

pub trait OptionExt {
    fn as_kind(&self) -> Option<Token>;
}

impl OptionExt for Option<Spanned<Token>> {
    fn as_kind(&self) -> Option<Token> {
        self.as_ref().map(|t| t.kind)
    }
}

impl OptionExt for Option<&Spanned<Token>> {
    fn as_kind(&self) -> Option<Token> {
        self.as_ref().map(|t| t.kind)
    }
}
