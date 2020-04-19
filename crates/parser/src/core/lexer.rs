use crate::core::{Input, Spanned, TextRange};
use crate::{Token};

pub struct LexerState {
    input: Input,
    #[allow(clippy::option_option)]
    peeked: Option<Option<Spanned<Token>>>,
}

impl LexerState {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.into(),
            peeked: None,
        }
    }
}

pub struct LexerIter<Lex: Lexer> {
    lexer: Lex
}

impl<Lex> Iterator for LexerIter<Lex> where Lex: Lexer {
    type Item = Spanned<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        self.lexer.next()
    }
}

pub trait Lexer {
    fn into_iter(self) -> LexerIter<Self> where Self: Sized { LexerIter { lexer: self } }
    //TODO: Split into state/state_mut
    fn state_mut(&mut self) -> &mut LexerState;
    fn state(&self) -> &LexerState;
    fn input(&self) -> Input {
        self.state().input.clone()
    }

    fn lex(input: &mut Input) -> Option<(Token, TextRange)>;
    //TODO: doc hidden
    fn next_token(&mut self) -> Option<Spanned<Token>> {
        if let Some(peeked) = self.state_mut().peeked.take() {
            if let Some(peeked) = peeked.as_ref() {
                self.state_mut().input.cursor = peeked.span.end();
            }
            return peeked;
        }

        let (token, span) = Self::lex(&mut self.state_mut().input)?;
        Some(Spanned::new(span, token))
    }
    fn next(&mut self) -> Option<Spanned<Token>> {
        let mut first = self.next_token()?;

        while let Token::Error = first.kind {
            match self.peek() {
                Some(token) if token.kind == Token::Error => {
                    first.span = TextRange::covering(first.span, token.span);
                    self.next_token();
                }
                _ => break,
            }
        }
        Some(first)
    }
    fn peek(&mut self) -> Option<&Spanned<Token>> {
        if self.state_mut().peeked.is_none() {
            let i = self.state_mut().input.cursor;
            self.state_mut().peeked = Some(self.next());
            self.state_mut().input.cursor = i;
        }

        self.state_mut().peeked.as_ref().and_then(|i| i.as_ref())
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
