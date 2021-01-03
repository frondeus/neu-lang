use crate::{Trivia, TokenKind};

pub struct Context<'source, 'ctx, Tok: TokenKind<'source>> {
    pub leading_trivia: Option<&'ctx dyn Trivia<'source, Tok>>,
    pub trailing_trivia: Option<&'ctx dyn Trivia<'source, Tok>>,
}

impl<'source, 'ctx, Tok: TokenKind<'source>> Default for Context<'source, 'ctx, Tok> {
    fn default() -> Self {
        Self {
            leading_trivia: None,
            trailing_trivia: None,
        }
    }
}

impl<'source, 'ctx, Tok: TokenKind<'source>> Context<'source, 'ctx, Tok> {
    pub fn new(trivia: &'ctx dyn Trivia<'source, Tok>) -> Self {
        Self {
            leading_trivia: Some(trivia),
            trailing_trivia: Some(trivia),
        }
    }

    pub fn leading_trivia(&self) -> Option<&'ctx dyn Trivia<'source, Tok>> {
        self.leading_trivia
    }

    pub fn trailing_trivia(&self) -> Option<&'ctx dyn Trivia<'source, Tok>> {
        self.trailing_trivia
    }
}
