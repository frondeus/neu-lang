use crate::core::{Parser, TokenKind};

pub struct Context<'a, Tok: TokenKind> {
    trivia: Option<&'a dyn Parser<Tok>>,
}

impl<'a, Tok: TokenKind> Default for Context<'a, Tok> {
    fn default() -> Self {
        Self { trivia: None }
    }
}

impl<'a, Tok: TokenKind> Context<'a, Tok> {
    pub fn new(trivia: &'a dyn Parser<Tok>) -> Self {
        Self {
            trivia: Some(trivia),
        }
    }

    pub fn trivia(&self) -> Option<&'a dyn Parser<Tok>> {
        self.trivia
    }
}
