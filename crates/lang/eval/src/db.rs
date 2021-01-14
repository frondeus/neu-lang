#![allow(dead_code)]
use crate::result::EvalResult;
use crate::Eval;
use neu_canceled::Canceled;
use neu_syntax::ast::ArticleItem;
use neu_syntax::reexport::{Red, Green, ParseResult};
use neu_syntax::db::{FileId, Parser};
use std::sync::Arc;

#[salsa::query_group(EvaluatorDatabase)]
pub trait Evaluator: salsa::Database + Parser {
    fn eval(&self, red: Red) -> Arc<EvalResult>;
    fn eval_file(&self, file: FileId) -> Arc<EvalResult>;
    fn anchored(&self, file: FileId) -> Arc<ParseResult>;
}

fn anchored(db: &dyn Evaluator, file: FileId) -> Arc<ParseResult> {
    Canceled::cancel_if(db.salsa_runtime());
    /*
    let parsed = db.parse_syntax(file);
    let mut parsed = (*parsed).clone();
    let items = parsed
        .arena
        .enumerate()
        .map(|(id, _)| id)
        .filter_map(|id| ArticleItem::from_syntax(id, &parsed.arena))
        .collect::<Vec<_>>();

    items.into_iter().for_each(|ast| {
        ast.anchor_body(&mut parsed.arena);
    });
    Arc::new(parsed)
    */
    todo!();
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

    Arc::new(EvalResult {
        value, errors
    })
    /*
    let input = db.input(file);
    let parsed = db.anchored(file);
    let mut eval = Eval::new(&parsed.arena, &input);
    let value = eval.eval(id).and_then(|val| eval.into_eager(val, true));
    let errors = eval.errors;
    */
}
