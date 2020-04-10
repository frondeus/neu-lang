use crate::Token;
use std::fmt::{Debug, Display, Formatter, Result};
use text_size::TextRange;

#[derive(PartialEq)]
pub struct Spanned<K> {
    pub kind: K,
    pub span: TextRange,
}

impl Spanned<Token> {
    pub fn new(span: TextRange, kind: Token) -> Self {
        Self { kind, span }
    }

    pub fn display<'k, 's>(&'k self, str: &'s str) -> DisplaySpanned<'k, 's, Token> {
        DisplaySpanned {
            str,
            kind: &self.kind,
            span: self.span,
        }
    }
}

impl<K> Debug for Spanned<K>
where
    K: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}@{:?}", self.kind, self.span)
    }
}

pub struct DisplaySpanned<'k, 's, K> {
    str: &'s str,
    kind: &'k K,
    span: TextRange,
}

impl<'k, 's, K> Display for DisplaySpanned<'k, 's, K>
where
    K: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?} `{}`", self.kind, &self.str[self.span])
    }
}
