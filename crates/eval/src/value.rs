use std::collections::HashMap;
use std::fmt;
use itertools::Itertools;

#[derive(Debug)]
pub enum Value {
    Number(i64),
    Boolean(bool),
    String(String),
    Array(Vec<Value>),
    Struct(HashMap<String, Value>)
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{}", n),
            Self::Boolean(b) => write!(f, "{}", b),
            Self::String(s) => write!(f, "{:?}", s),
            Self::Array(a) => write!(f, "[{}]", a.iter().map(|v| v.to_string()).join(", ")),
            Self::Struct(s) => {
                writeln!(f, "{{")?;
                for (k, v) in s.iter() {
                    writeln!(f, "{} = {},", k, v)?;
                }
                writeln!(f, "}}")
            }
        }
    }
}
