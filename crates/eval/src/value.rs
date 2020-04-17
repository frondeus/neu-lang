use std::collections::BTreeMap;
use std::fmt;
use itertools::Itertools;

#[derive(Debug)]
pub enum Value {
    Number(i64),
    Boolean(bool),
    String(String),
    Array(Vec<Value>),
    Struct(BTreeMap<String, Value>)
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width = f.width().unwrap_or_default();
        //if width > 0 { write!(f, "{:width$}", " ", width = width)?; }
        match self {
            Self::Number(n) => write!(f, "{}", n),
            Self::Boolean(b) => write!(f, "{}", b),
            Self::String(s) => write!(f, "{:?}", s),
            Self::Array(a) => write!(f, "[{}]", a.iter().map(|v| v.to_string()).join(", ")),
            Self::Struct(s) => {
                writeln!(f, "{{")?;
                let c_width = width + 4;
                for (k, v) in s.iter() {
                    writeln!(f, "{:width$}{} = {:width$},", " ", k, v, width = c_width)?;
                }
                write!(f, "{:width$}}}", " ", width = width)
            }
        }
    }
}
