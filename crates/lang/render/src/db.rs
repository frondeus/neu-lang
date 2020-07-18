#![allow(dead_code)]
use crate::_render;
use crate::result::RenderResult;
use neu_analyze::db::Analyzer;
use neu_syntax::ast::{ArticleItem, RootAst};
use neu_syntax::db::{FileId, Parser};

#[salsa::query_group(RendererDatabase)]
pub trait Renderer: salsa::Database + Parser + Analyzer {
    fn render_md(&self, path: FileId) -> RenderResult;
    fn render_item(&self, kind: String, id: String) -> RenderResult;
    fn render_ast(&self, path: FileId, article_item: ArticleItem) -> RenderResult;
}

fn render_md(db: &dyn Renderer, path: FileId) -> RenderResult {
    log::debug!("Rendering {}", &path);
    let parsed = db.parse_md_syntax(path.clone());
    let article_item = ArticleItem::from_root_syntax(&parsed);

    db.render_ast(path, article_item)
}

fn render_ast(db: &dyn Renderer, path: FileId, article_item: ArticleItem) -> RenderResult {
    //log::info!("Rendering {}:{} - {}", kind, id, title);

    let input = db.input_md(path.clone());
    let mut parsed = db.parse_md_syntax(path);

    let rendered = _render(db, article_item, &mut parsed.arena, &input);

    RenderResult {
        output: rendered,
        arena: parsed.arena,
    }
}

fn render_item(db: &dyn Renderer, kind: String, id: String) -> RenderResult {
    let article = db.find_md(kind.clone(), id.clone());

    let (path, article_item) = match article {
        Some(a) => a,
        None => {
            return RenderResult {
                output: format!("Couldn't render, article {}:{} not found", kind, id),
                arena: Default::default(),
            }
        }
    };

    db.render_ast(path, article_item)
}
