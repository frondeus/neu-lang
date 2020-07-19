#![allow(dead_code)]
use crate::result::EvalResult;
use crate::Eval;
use neu_canceled::Canceled;
use neu_parser::{NodeId, ParseResult};
use neu_syntax::ast::{ArticleItem, Ast};
use neu_syntax::db::{FileId, Parser};

#[salsa::query_group(EvaluatorDatabase)]
pub trait Evaluator: salsa::Database + Parser {
    fn eval(&self, file: FileId, id: NodeId) -> EvalResult;
    fn anchored(&self, file: FileId) -> ParseResult;
}

fn anchored(db: &dyn Evaluator, file: FileId) -> ParseResult {
    Canceled::cancel_if(db.salsa_runtime());
    let mut parsed = db.parse_syntax(file);
    let items = parsed
        .arena
        .enumerate()
        .map(|(id, _)| id)
        .filter_map(|id| ArticleItem::from_syntax(id, &parsed.arena))
        .collect::<Vec<_>>();

    items.into_iter().for_each(|ast| {
        ast.anchor_body(&mut parsed.arena);
    });
    parsed
}

fn eval(db: &dyn Evaluator, file: FileId, id: NodeId) -> EvalResult {
    Canceled::cancel_if(db.salsa_runtime());
    let input = db.input(file.clone());
    let parsed = db.anchored(file);
    let mut arena = parsed.arena;
    let mut eval = Eval::new(&arena, &input);
    let value = eval.eval(id).and_then(|val| eval.into_eager(val, true));
    let new_arena = eval.new_arena;
    arena.merge(new_arena);
    EvalResult {
        value, //errors: eval.errors,
        arena,
    }
}
