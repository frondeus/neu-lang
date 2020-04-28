use derive_more::Display;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(inspectable)]
pub struct Diagnostic {
    text: String,
    pub line: i32,
    pub end_line: i32,
    pub col: i32,
    pub end_col: i32,
    pub typ: DiagnosticType
}

impl Diagnostic {
    pub fn new(text: impl Into<String>, line: i32, col: i32, end_line: i32, end_col: i32, typ: DiagnosticType) -> Self {
        Self {
            text: text.into(),
            line: line + 1,
            col: col + 1,
            end_line: end_line + 1,
            end_col: end_col + 1,
            typ
        }
    }
}

#[wasm_bindgen]
impl Diagnostic {
    #[wasm_bindgen(getter)]
    pub fn text(&self) -> String {self.text.clone()}

    #[wasm_bindgen(getter)]
    pub fn severity(&self) -> u64 {
        self.typ as u64
    }
}

#[wasm_bindgen]
#[derive(Display, Clone, Copy)]
pub enum DiagnosticType {
    #[display(fmt = "error")]
    Error = 8,
    #[display(fmt = "warning")]
    Warning = 4
}

