#![allow(dead_code)]
use crate::_render;
use crate::result::RenderResult;
use neu_analyze::db::Analyzer;
use neu_canceled::Canceled;
use neu_eval::db::Evaluator;
use neu_syntax::{ast::{ArticleItem, MainArticle}, reexport::{Ast, Red}};
use neu_syntax::db::{FileId, Parser};
use std::sync::Arc;

#[salsa::query_group(RendererDatabase)]
pub trait Renderer: salsa::Database + Parser + Analyzer + Evaluator {
    fn render_md(&self, path: FileId) -> Arc<RenderResult>;
    fn render_item(&self, kind: String, id: String) -> Arc<RenderResult>;
    fn render_ast(&self, article_item: ArticleItem) -> Arc<RenderResult>;
}

fn render_md(db: &dyn Renderer, path: FileId) -> Arc<RenderResult> {
    Canceled::cancel_if(db.salsa_runtime());
    log::debug!("Rendering {:?}", &path);
    let parsed = db.parse_syntax(path);
    let red = Red::root(parsed.root.clone());
    let main = MainArticle::new(red);

    let article_item: ArticleItem = match main {
        Some(m) => m.into(),
        None => {
            return Arc::new(RenderResult {
                output: format!("Couldn't render, file {:?} is not an article", &path),
                ..Default::default()
            })
        }
    };

    db.render_ast(article_item)
}

fn render_ast(db: &dyn Renderer, article_item: ArticleItem) -> Arc<RenderResult> {
    Canceled::cancel_if(db.salsa_runtime());

    let mut result = RenderResult::default();

    _render(db, article_item, &mut result);

    Arc::new(result)
}

fn render_item(db: &dyn Renderer, kind: String, id: String) -> Arc<RenderResult> {
    Canceled::cancel_if(db.salsa_runtime());
    let article = db.find_md(kind.clone(), id.clone());

    let (path, article_item) = match article {
        Some(a) => a,
        None => {
            return Arc::new(RenderResult {
                output: format!("Couldn't render, article {}:{} not found", kind, id),
                ..Default::default()
            });
        }
    };

    db.render_ast(article_item)
}
