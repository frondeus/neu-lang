mod span_ext;
mod diagnostic;

use wasm_bindgen::prelude::*;
use web_sys::console;
use crate::diagnostic::{Diagnostic, DiagnosticType};
use crate::span_ext::{TextRangeExt, LinesCols};
use neu_parser::State;
use neu_syntax::{neu::parser, MainLexer};

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    console::log_1(&JsValue::from_str("Neu-lang: hi there!"));

    Ok(())
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = neu)]
    fn clear_diagnostics();

    #[wasm_bindgen(js_namespace = neu)]
    fn write_eval(val: String);

    #[wasm_bindgen(js_namespace = neu)]
    fn write_diagnostic(diagnostic: Diagnostic);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}


#[wasm_bindgen]
pub fn on_change(buf: &str) {
    let lines = buf.lines().map(|s| s.to_string()).collect::<Vec<_>>();

    let parse_result = State::parse(MainLexer::new(buf), parser());
    //log(&format!("{:?}", &parse_result));

    clear_diagnostics();

    let mut diagnostics = parse_result.errors.iter().map(|(id, error)| {
        let node = parse_result.nodes.get(id);
        let LinesCols { line_start, line_end, col_start, col_end } = node.span.lines_cols(&lines);
        Diagnostic::new(error.display(&buf).to_string(),
                             line_start,
                             col_start,
                             line_end,
                             col_end,
                             DiagnosticType::Error
        )
    }).collect::<Vec<Diagnostic>>();

    let root = parse_result.root;

    let root_eval_result = neu_eval::eval(root, &parse_result.nodes, &buf);
    //log(&format!("{}", &root_eval_result.display(buf)));

    if let Some(value) = root_eval_result.value {
        write_eval(format!("= {}", &value));
    }

    for (id, error) in root_eval_result.errors.iter() {
        let node = parse_result.nodes.get(id);
        let LinesCols { line_start, line_end, col_start, col_end } = node.span.lines_cols(&lines);
        diagnostics.push(Diagnostic::new(error.to_string(),
                                         line_start,
                                         col_start,
                                         line_end,
                                         col_end,
                                         DiagnosticType::Error));
    }

    for diagnostic in diagnostics {
        write_diagnostic(diagnostic);
    }
}
