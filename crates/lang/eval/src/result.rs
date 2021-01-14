use crate::value::Value;
use std::fmt;
use neu_diagnostics::Diagnostics;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EvalResult {
    pub value: Option<Value>,
    pub errors: Diagnostics
}

impl fmt::Display for EvalResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.value {
            Some(v) => write!(f, "`{:#}`", v),
            None => write!(f, "None")
        }
    }
}
