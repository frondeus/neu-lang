use neu_parser::Arena;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RenderResult {
    pub output: String,
    pub arena: Arena,
}

impl RenderResult {
    pub fn display<'s, 'n>(&'n self, str: &'s str) -> DisplayRenderResult<'s, 'n> {
        DisplayRenderResult { str, result: self }
    }
}

pub struct DisplayRenderResult<'s, 'n> {
    #[allow(dead_code)]
    str: &'s str,
    result: &'n RenderResult,
}

impl<'s, 'n> fmt::Display for DisplayRenderResult<'s, 'n> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.result.output)?;
        Ok(())
    }
}
