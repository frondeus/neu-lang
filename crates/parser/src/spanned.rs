use std::fmt::{Debug, Display, Formatter, Result};
use text_size::TextRange;
use crate::TokenKind;

#[derive(PartialEq, Clone)]
pub struct Spanned<K> {
    pub kind: K,
    pub span: TextRange,
}

impl<Tok: TokenKind> Spanned<Tok> {
    pub fn new(span: TextRange, kind: Tok) -> Self {
        Self { kind, span }
    }

    pub fn display<'k, 's>(&'k self, str: &'s str, display_kind: bool) -> DisplaySpanned<'k, 's, Tok> {
        DisplaySpanned {
            str,
            kind: &self.kind,
            span: self.span,
            display_kind
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
    display_kind: bool
}

impl<'k, 's, K> Display for DisplaySpanned<'k, 's, K>
where
    K: Debug + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if self.display_kind {
            write!(f, "{:?} ", self.kind)?
        }
        write!(f, "`{}`", &self.str[self.span])
    }
}
