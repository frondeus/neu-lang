use crate::core::{Parser, Lexer};

pub struct Context<'a, Lex: Lexer> {
    trivia: Option<&'a dyn Parser<Lex>>,
}

impl<'a, Lex: Lexer> Default for Context<'a, Lex> {
    fn default() -> Self {
        Self { trivia: None }
    }
}

impl<'a, Lex: Lexer> Context<'a, Lex> {
    pub fn new(trivia: &'a dyn Parser<Lex>) -> Self {
        Self {
            trivia: Some(trivia),
        }
    }

    pub fn trivia(&self) -> Option<&'a dyn Parser<Lex>> {
        self.trivia
    }
}
