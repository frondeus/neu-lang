use neu_eval::Value;
use std::fmt;

pub(crate) struct HtmlValue<'v> {
    value: &'v Value,
}

impl<'v> fmt::Display for HtmlValue<'v> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width = f.width().unwrap_or_default();
        match self.value {
            Value::Number(n) => write!(f, "{}", n),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::String(s) => write!(f, "{}", s),
            Value::Array(a) => {
                write!(f, "[")?;
                if !a.is_empty() {
                    if let Some(v) = a.iter().next() {
                        write!(f, " {}", v)?;
                    }
                    for v in a.iter().skip(1) {
                        write!(f, ", {}", v)?;
                    }
                    write!(f, " ")?;
                }
                write!(f, "]")
            }
            Value::Struct(s) => {
                write!(f, "{{")?;
                if !s.is_empty() {
                    writeln!(f)?;
                }
                let c_width = width + 4;
                for (k, v) in s.iter() {
                    writeln!(f, "{:width$}{} = {:#width$},", " ", k, v, width = c_width)?;
                }
                write!(f, "{:width$}}}", " ", width = width)
            }
            Value::Lazy { .. } => unreachable!("lazy render"),
        }
    }
}

pub(crate) fn render_value(value: &Value) -> HtmlValue {
    HtmlValue { value }
}
