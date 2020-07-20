#![allow(dead_code)]
use crate::result::EvalResult;
use crate::Eval;
use neu_canceled::Canceled;
use neu_parser::{NodeId, ParseResult};
use neu_syntax::ast::{ArticleItem, Ast};
use neu_syntax::db::{FileId, Parser};
use std::sync::Arc;

#[salsa::query_group(EvaluatorDatabase)]
pub trait Evaluator: salsa::Database + Parser {
    fn eval(&self, file: FileId, id: NodeId) -> Arc<EvalResult>;
    fn anchored(&self, file: FileId) -> Arc<ParseResult>;
}

fn anchored(db: &dyn Evaluator, file: FileId) -> Arc<ParseResult> {
    Canceled::cancel_if(db.salsa_runtime());
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
}

fn eval(db: &dyn Evaluator, file: FileId, id: NodeId) -> Arc<EvalResult> {
    Canceled::cancel_if(db.salsa_runtime());
    let input = db.input(file);
    let parsed = db.anchored(file);
    let mut eval = Eval::new(&parsed.arena, &input);
    let value = eval.eval(id).and_then(|val| eval.into_eager(val, true));
    let errors = eval.errors;
    Arc::new(EvalResult {
        value,
        errors
    })
}
