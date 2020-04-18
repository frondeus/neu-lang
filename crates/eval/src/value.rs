use std::collections::BTreeMap;
use std::fmt;
use neu_parser::core::NodeId;

#[derive(Debug, Clone)]
pub enum Value {
    Number(i64),
    Boolean(bool),
    String(String),
    Array(Vec<Value>),
    Struct(BTreeMap<String, Value>),

    Lazy { id: NodeId, parent: NodeId }
}

impl Value {
    pub fn is_lazy(&self) -> bool {
        match self {
            Self::Lazy{..} => true,
            _ => false
        }
    }

    pub fn into_struct(self) -> Option<BTreeMap<String, Value>> {
        match self {
            Self::Struct(s) => Some(s),
            _ => None
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width = f.width().unwrap_or_default();
        match self {
            Self::Lazy { id, .. } => write!(f, "{:?}", id),

            Self::Number(n) => write!(f, "{}", n),
            Self::Boolean(b) => write!(f, "{}", b),
            Self::String(s) => write!(f, "{:?}", s),
            Self::Array(a) => {
                write!(f, "[")?;
                if !a.is_empty() {
                    if let Some(v) = a.iter().next() { write!(f, " {}", v)?; }
                    for v in a.iter().skip(1) { write!(f, ", {}", v)?; }
                    write!(f, " ")?;
                }
                write!(f, "]")
            }
            Self::Struct(s) => {
                if f.alternate() {
                    writeln!(f, "{{")?;
                    let c_width = width + 4;
                    for (k, v) in s.iter() {
                        writeln!(f, "{:width$}{} = {:#width$},", " ", k, v, width = c_width)?;
                    }
                    write!(f, "{:width$}}}", " ", width = width)
                } else {
                    write!(f, "{{")?;
                    if !s.is_empty() {
                        if let Some((k, v)) = s.iter().next() { write!(f, " {} = {}", k, v)?; }
                        for (k, v) in s.iter().skip(1) { write!(f, ", {} = {}", k, v)?; }
                        write!(f, " ")?;
                    }
                    write!(f, "}}")
                }
            }
        }
    }
}
