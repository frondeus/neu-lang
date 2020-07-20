use crate::value::Value;
use neu_parser::{NodeId};
use std::fmt;
use neu_diagnostics::Diagnostics;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EvalResult {
    pub value: Option<Value>,
    pub errors: Diagnostics<NodeId>
}

impl EvalResult {
    pub fn display<'s, 'n>(&'n self, str: &'s str) -> DisplayEvalResult<'s, 'n> {
        DisplayEvalResult { str, result: self }
    }
}

pub struct DisplayEvalResult<'s, 'n> {
    #[allow(dead_code)]
    str: &'s str,
    result: &'n EvalResult,
}

impl<'s, 'n> fmt::Display for DisplayEvalResult<'s, 'n> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.result.value {
            None => write!(f, "None")?,
            Some(r) => write!(f, "`{:#}`", r)?,
        };
        Ok(())
    }
}
