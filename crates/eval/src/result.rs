use crate::value::Value;
use crate::error::Error;
use neu_parser::NodeId;
use std::fmt;

pub struct EvalResult {
    pub value: Option<Value>,
    pub errors: Vec<(NodeId, Error)>
}

impl EvalResult {
    pub fn display<'s, 'n>(&'n self, str: &'s str) -> DisplayEvalResult<'s, 'n> {
        DisplayEvalResult { str, result: self }
    }
}

pub struct DisplayEvalResult<'s, 'n> {
    #[allow(dead_code)]
    str: &'s str,
    result: &'n EvalResult
}

impl<'s, 'n> fmt::Display for DisplayEvalResult<'s, 'n> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.result.value {
            None => write!(f, "None")?,
            Some(r) => write!(f, "`{:#}`", r)?
        };
        if self.result.errors.is_empty() {
            write!(f, "\n\n### No Errors ###")?;
        }
        else {
            write!(f, "\n\n### Errors ###")?;
        }

        for (node_id, error) in self.result.errors.iter() {
            write!(f, "\n{} @ {:?}", error, node_id)?;
        }

        Ok(())
    }
}
