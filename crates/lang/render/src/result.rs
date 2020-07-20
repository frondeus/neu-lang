use neu_parser::{NodeId, Diagnostics};
use std::fmt;

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct RenderResult {
    pub output: String,
    pub errors: Diagnostics<NodeId>
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
