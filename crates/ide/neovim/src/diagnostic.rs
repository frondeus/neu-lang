use nvim_rs::rpc::model::IntoVal;
use crate::Buffer;
use nvim_rs::Value;
use derive_more::Display;

pub struct Diagnostic {
    bufnr: Buffer,
    text: String,
    line: i64,
    col: i64,
    typ: DiagnosticType
}

impl Diagnostic {
    pub fn new(bufnr: &Buffer, text: impl Into<String>, line: i64, col: i64, typ: DiagnosticType) -> Self {
        Self {
            bufnr: bufnr.clone(),
            text: text.into(),
            line,
            col,
            typ
        }
    }

    pub fn line(&self) -> i64 { self.line }

    pub fn text(&self) -> String { format!("# {}: {}", self.typ, &self.text) }
}

#[derive(Display)]
pub enum DiagnosticType {
    #[display(fmt = "error")]
    Error,
    #[display(fmt = "warning")]
    Warning
}

impl IntoVal<Value> for DiagnosticType {
    fn into_val(self) -> Value {
        match self {
            Self::Error => "E".into_val(),
            Self::Warning => "W".into_val(),
        }
    }
}

impl IntoVal<Value> for Diagnostic {
    fn into_val(self) -> Value {
        Value::Map(vec![
            ("bufnr".into_val(), self.bufnr.into_val()),
            ("text".into_val(), self.text.into_val()),
            ("lnum".into_val(), (self.line + 1).into_val()),
            ("col".into_val(),  (self.col + 1).into_val()),
            ("type".into_val(), self.typ.into_val()),
        ])
    }
}

