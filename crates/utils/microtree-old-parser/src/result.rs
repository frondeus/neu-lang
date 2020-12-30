use std::fmt::Display;

use crate::Error;
use microtree::Green;

#[derive(Debug, Eq, PartialEq)]
pub struct ParseResult {
    pub root: Option<Green>,
    pub errors: Vec<Error>,
}

impl Display for ParseResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.errors.is_empty() {
            writeln!(f, "Errors")?;
            for error in &self.errors {
                writeln!(f, "* {:?}", error)?;
            }
        }
        if let Some(root) = self.root.as_ref() {
            write!(f, "{:?}", root)?;
        } else {
            write!(f, "No Root")?;
        }
        Ok(())
    }
}
