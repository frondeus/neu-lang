use crate::core::{Input, Spanned, TextRange};

pub struct LexerState<Tok: TokenKind> {
    input: Input,
    #[allow(clippy::option_option)]
    peeked: Option<Option<Spanned<Tok>>>,
}

impl<Tok: TokenKind> LexerState<Tok> {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.into(),
            peeked: None,
        }
    }
}

pub struct LexerIter<Lex: Lexer<Token = T>, T: TokenKind> {
    lexer: Lex
}

impl<Lex, T> Iterator for LexerIter<Lex, T> where Lex: Lexer<Token = T>, T: TokenKind {
    type Item = Spanned<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.lexer.next()
    }
}

pub trait TokenKind: Clone + Copy + std::fmt::Debug + std::fmt::Display + PartialEq {
    fn is_error(&self) -> bool;
}

pub trait Lexer {
    type Token: TokenKind;

    fn into_iter(self) -> LexerIter<Self, Self::Token> where Self: Sized { LexerIter { lexer: self } }

    //TODO: Split into state/state_mut
    fn state_mut(&mut self) -> &mut LexerState<Self::Token>;
    fn state(&self) -> &LexerState<Self::Token>;
    fn input(&self) -> Input {
        self.state().input.clone()
    }

    fn lex(input: &mut Input) -> Option<(Self::Token, TextRange)>;
    //TODO: doc hidden
    fn next_token(&mut self) -> Option<Spanned<Self::Token>> {
        if let Some(peeked) = self.state_mut().peeked.take() {
            if let Some(peeked) = peeked.as_ref() {
                self.state_mut().input.cursor = peeked.span.end();
            }
            return peeked;
        }

        let (token, span) = Self::lex(&mut self.state_mut().input)?;
        Some(Spanned::new(span, token))
    }
    fn next(&mut self) -> Option<Spanned<Self::Token>> {
        let mut first = self.next_token()?;

        while first.kind.is_error() {
            match self.peek() {
                Some(token) if token.kind.is_error() => {
                    first.span = TextRange::covering(first.span, token.span);
                    self.next_token();
                }
                _ => break,
            }
        }
        Some(first)
    }
    fn peek(&mut self) -> Option<&Spanned<Self::Token>> {
        if self.state_mut().peeked.is_none() {
            let i = self.state_mut().input.cursor;
            self.state_mut().peeked = Some(self.next());
            self.state_mut().input.cursor = i;
        }

        self.state_mut().peeked.as_ref().and_then(|i| i.as_ref())
    }
}

pub trait OptionExt<T> {
    fn as_kind(&self) -> Option<T>;
}

impl<T: Copy> OptionExt<T> for Option<Spanned<T>> {
    fn as_kind(&self) -> Option<T> {
        self.as_ref().map(|t| t.kind)
    }
}

impl<T: Copy> OptionExt<T> for Option<&Spanned<T>> {
    fn as_kind(&self) -> Option<T> {
        self.as_ref().map(|t| t.kind)
    }
}
