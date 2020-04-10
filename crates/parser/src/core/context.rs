use crate::core::Parser;

#[derive(Default)]
pub struct Context<'a> {
    trivia: Option<&'a dyn Parser>,
}

impl<'a> Context<'a> {
    pub fn new(trivia: &'a dyn Parser) -> Self {
        Self {
            trivia: Some(trivia),
        }
    }

    pub fn trivia(&self) -> Option<&'a dyn Parser> {
        self.trivia
    }
}
