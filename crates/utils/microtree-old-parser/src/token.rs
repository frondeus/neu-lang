use std::fmt::Debug;

use crate::{SmolStr, TextRange};

#[derive(Clone)]
pub struct Spanned<Tok> {
    pub token: Tok,
    pub value: SmolStr,
    pub range: TextRange,
}

impl<Tok: Debug> Debug for Spanned<Tok> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} `{}`", self.token, self.value)
    }
}
