#![allow(dead_code)]
use crate::_render;
use crate::result::RenderResult;
use neu_analyze::db::Analyzer;
use neu_canceled::Canceled;
use neu_eval::db::Evaluator;
use neu_syntax::ast::{ArticleItem, RootAst};
use neu_syntax::db::{FileId, Parser};
use std::sync::Arc;

#[salsa::query_group(RendererDatabase)]
pub trait Renderer: salsa::Database + Parser + Analyzer + Evaluator {
    fn render_md(&self, path: FileId) -> Arc<RenderResult>;
    fn render_item(&self, kind: String, id: String) -> Arc<RenderResult>;
    fn render_ast(&self, path: FileId, article_item: ArticleItem) -> Arc<RenderResult>;
}

fn render_md(db: &dyn Renderer, path: FileId) -> Arc<RenderResult> {
    Canceled::cancel_if(db.salsa_runtime());
    log::debug!("Rendering {:?}", &path);
    let parsed = db.parse_syntax(path);
    let article_item = ArticleItem::from_root_syntax(&parsed);

    db.render_ast(path, article_item)
}

fn render_ast(db: &dyn Renderer, path: FileId, article_item: ArticleItem) -> Arc<RenderResult> {
    Canceled::cancel_if(db.salsa_runtime());
    //log::info!("Rendering {}:{} - {}", kind, id, title);

    let input = db.input(path);
    let parsed = db.parse_syntax(path);
    let mut arena = parsed.arena.clone();

    let rendered = _render(db, path, article_item, &mut arena, &input);

    Arc::new(RenderResult {
        output: rendered,
        arena,
    })
}

fn render_item(db: &dyn Renderer, kind: String, id: String) -> Arc<RenderResult> {
    Canceled::cancel_if(db.salsa_runtime());
    let article = db.find_md(kind.clone(), id.clone());

    let (path, article_item) = match article {
        Some(a) => a,
        None => {
            return Arc::new(RenderResult {
                output: format!("Couldn't render, article {}:{} not found", kind, id),
                arena: Default::default(),
            });
        }
    };

    db.render_ast(path, article_item)
}
