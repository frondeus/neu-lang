use crate::value::Value;
use neu_parser::{NodeId, Arena, ArenaExt};
use std::fmt;
use neu_diagnostics::Diagnostic;

pub struct EvalResult {
    pub value: Option<Value>,
}

impl EvalResult {
    pub fn display<'s, 'n>(&'n self, str: &'s str, arena: &'s Arena) -> DisplayEvalResult<'s, 'n> {
        DisplayEvalResult { str, result: self, arena}
    }
}

pub struct DisplayEvalResult<'s, 'n> {
    #[allow(dead_code)]
    str: &'s str,
    result: &'n EvalResult,
    arena: &'s Arena
}

impl<'s, 'n> fmt::Display for DisplayEvalResult<'s, 'n> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.result.value {
            None => write!(f, "None")?,
            Some(r) => write!(f, "`{:#}`", r)?,
        };
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
