use neu_diagnostics::Diagnostics;
use std::fmt;

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct RenderResult {
    pub output: String,
    pub errors: Diagnostics
}

impl<'s, 'n> fmt::Display for RenderResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.output)?;
        Ok(())
    }
}
