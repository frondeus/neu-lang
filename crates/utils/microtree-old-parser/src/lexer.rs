use crate::{PeekableIterator, TextRange, TextSize, Spanned};
use logos::Logos;
use logos::Lexer as Inner;
use smol_str::SmolStr;


pub trait TokenKind<'source> : Logos<'source, Source = str, Extras: Clone>
    + std::fmt::Display + PartialEq
    + Clone
    + Copy
    + Send
    + Sync
{
    fn mergeable(self, _other: Self) -> bool { false }
}

#[derive(Clone)]
pub struct Lexer<'source, Tok: TokenKind<'source>> {
    #[allow(clippy::option_option)]
    peeked: Option<Option<(Inner<'source, Tok>, Spanned<Tok>)>>,
    inner: Inner<'source, Tok>
}


impl<'source, Tok: TokenKind<'source>> Lexer<'source, Tok> {
    pub fn new(source: &'source Tok::Source) -> Self {
        Self {
            inner: Inner::new(source),
            peeked: None
        }
    }

    pub fn morph<Tok2>(self) -> Lexer<'source, Tok2>
    where Tok2: TokenKind<'source>,
        Tok::Extras: Into<Tok2::Extras>,
    {
        Lexer {
            peeked: None,
            inner: self.inner.morph()
        }
    }

    pub fn span(&self) -> TextRange {
        let range = self.inner.span();
        TextRange::new((range.start as u32).into(), (range.end as u32).into())
    }

    pub(crate) fn text_for_span(&self, span: TextRange) -> SmolStr {
        let source = self.inner.source();
        source[span].into()
    }

    pub fn peek_token(&mut self) -> Option<Tok> {
        self.peek().map(|t| t.token)
    }
}

impl<'source, Tok> PeekableIterator for Lexer<'source, Tok>
where Tok: TokenKind<'source>
{
    fn peek(&mut self) -> Option<&Self::Item> {
        if self.peeked.is_none() {
            let saved = self.inner.clone();
            let token = self.next();
            let original = std::mem::replace(&mut self.inner, saved);
            self.peeked = Some(token.map(|token| (original, token)));
        }

        self.peeked.as_ref().and_then(|t| t.as_ref())
            .map(|(_, t)| t)
    }
}

impl<'source, Tok: TokenKind<'source>> Iterator for Lexer<'source, Tok> {
    type Item = Spanned<Tok>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut first = self.lex()?;

        loop {
            match self.peek_one() {
                Some(token) if first.token.mergeable(token.token) => {
                    let from = first.range.start();
                    let len: TextSize = ((first.value.len() + token.value.len()) as u32).into();
                    let range = TextRange::at(from, len);
                    first.range = range;
                    let new_value = &self.inner.source()[range];
                    first.value = new_value.into();
                    self.lex();
                },
                _ => break Some(first)
            }
        }
    }
}

impl<'source, Tok: TokenKind<'source>> Lexer<'source, Tok> {
    fn lex(&mut self) -> Option<Spanned<Tok>> {
        if let Some(peeked) = self.peeked.take() {
            if let Some((original, peeked)) = peeked {
                self.inner = original;
                return Some(peeked);
            }
            return None;
        }
        let token = self.inner.next()?;
        let value = self.inner.slice().into();
        let range = self.span();
        Some(Spanned {
            token, range, value
        })
    }

    fn peek_one(&mut self) -> Option<&Spanned<Tok>> {
            if self.peeked.is_none() {
                let saved = self.inner.clone();
                let token = self.lex();
                let original = std::mem::replace(&mut self.inner, saved);
                self.peeked = Some(token.map(|token| (original, token)));
            }

            self.peeked.as_ref().and_then(|t| t.as_ref())
                .map(|(_, t)| t)
        }
}
