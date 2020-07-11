use neu_parser::{Arena, ArenaExt};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RenderResult {
    pub output: String,
}

impl RenderResult {
    pub fn display<'s, 'n>(&'n self, str: &'s str, arena: &'s Arena) -> DisplayRenderResult<'s, 'n> {
        DisplayRenderResult { str, result: self, arena }
    }
}

pub struct DisplayRenderResult<'s, 'n> {
    #[allow(dead_code)]
    str: &'s str,
    result: &'n RenderResult,
    arena: &'s Arena
}

impl<'s, 'n> fmt::Display for DisplayRenderResult<'s, 'n> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "`{}`", self.result.output)?;

        let errors = self.arena.errors();

        if errors.is_empty() {
            write!(f, "\n\n### No Errors ###")?;
        } else {
            write!(f, "\n\n### Errors ###")?;
        }

        for (node_id, error) in errors.iter() {
            write!(f, "\n{} @ {:?}", error.to_report(self.str), node_id)?;
        }
        Ok(())
    }
}
