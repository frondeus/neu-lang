#![allow(dead_code)]
use crate::result::EvalResult;
use crate::Eval;
use neu_canceled::Canceled;
use neu_syntax::reexport::Red;
use neu_syntax::db::{FileId, Parser};
use std::sync::Arc;

#[salsa::query_group(EvaluatorDatabase)]
pub trait Evaluator: salsa::Database + Parser {
    fn eval(&self, red: Red) -> Arc<EvalResult>;
    fn eval_file(&self, file: FileId) -> Arc<EvalResult>;
}

fn eval_file(db: &dyn Evaluator, file: FileId) -> Arc<EvalResult> {
    Canceled::cancel_if(db.salsa_runtime());

    let syntax = db.parse_syntax(file);
    let root = syntax.root.clone();
    let root = Red::root(root);
    db.eval(root)
}

fn eval(db: &dyn Evaluator, red: Red) -> Arc<EvalResult> {
    Canceled::cancel_if(db.salsa_runtime());

    let mut eval = Eval::new(red);
    let value = eval.eval()
        .and_then(|v| eval.into_eager(v, true));

    let errors = eval.errors;

    Arc::new(EvalResult { value, errors })
}
