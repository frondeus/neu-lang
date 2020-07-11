mod diagnostic;
mod span_ext;

use crate::diagnostic::{Diagnostic, DiagnosticType};
use crate::span_ext::{LinesCols, TextRangeExt};
use neu_parser::{ArenaExt, State};
use neu_syntax::{lexers::neu::Lexer, parsers::neu::parser};
use wasm_bindgen::prelude::*;
use web_sys::console;

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

    let parse_result = State::parse(Lexer::new(buf), parser());
    let mut arena = parse_result.arena;

    clear_diagnostics();

    let diagnostics = arena
        .errors()
        .into_iter()
        .map(|(id, error)| {
            let node = arena.get(id);
            let LinesCols {
                line_start,
                line_end,
                col_start,
                col_end,
            } = node.span.lines_cols(&lines);
            Diagnostic::new(
                error.to_report(&buf),
                line_start,
                col_start,
                line_end,
                col_end,
                DiagnosticType::Error,
            )
        })
        .collect::<Vec<Diagnostic>>();

    let root = parse_result.root;

    let root_eval_result = neu_eval::eval(root, &mut arena, &buf);
    //log(&format!("{}", &root_eval_result.display(buf)));

    if let Some(value) = root_eval_result.value {
        write_eval(format!("= {}", &value));
    }

    for diagnostic in diagnostics {
        write_diagnostic(diagnostic);
    }
}
