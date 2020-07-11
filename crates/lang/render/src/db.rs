#![allow(dead_code)]
use crate::result::RenderResult;
use neu_parser::{ParseResult, State};
use neu_syntax::lexers::article_item_file::Lexer;
use neu_syntax::parsers::article_item::parser;
use neu_syntax::ast::{ArticleItem, RootAst, Ast};
use crate::_render;
use std::collections::HashSet;

pub type FileId = String;

#[salsa::query_group(RendererDatabase)]
pub trait Renderer: salsa::Database {
    #[salsa::input]
    fn input_md(&self, path: FileId) -> String;

    #[salsa::input]
    fn all_mds(&self) -> HashSet<FileId>;

    fn parse_md_syntax(&self, path: FileId) -> ParseResult;
    fn render_md(&self, path: FileId) -> RenderResult;
    fn render_item(&self, kind: String, id: String) -> RenderResult;

                                    // kind, id,    path,    ast
    fn parse_all_mds(&self) -> Vec<(String, String, FileId, ArticleItem)>;
    fn find_md(&self, kind: String, id: String) -> Option<(FileId, ArticleItem)>;
}


fn parse_md_syntax(db: &dyn Renderer, path: FileId) -> ParseResult {
    let input = db.input_md(path);
    let input = input.as_str();
    let lexer = Lexer::new(&input);
    State::parse(lexer, parser())
}

fn render_md(db: &dyn Renderer, path: FileId) -> RenderResult {
    let input = db.input_md(path.clone());
    let mut parsed = db.parse_md_syntax(path);
    let article_item = ArticleItem::from_root_syntax(&parsed);

    let rendered = _render(db, article_item, &mut parsed.arena, &input)
        .unwrap_or_else(|| "Couldn't render, found errors".to_string());

    RenderResult {
        output: rendered,
        arena: parsed.arena
    }
}

fn parse_all_mds(db: &dyn Renderer) -> Vec<(String, String, FileId, ArticleItem)> {
    db.all_mds().into_iter().flat_map(|md| {
        let input = db.input_md(md.clone());
        let input = input.as_str();
        let parsed = db.parse_md_syntax(md.clone());

        parsed.arena.enumerate()
            .filter_map(|(id, _)| {
                ArticleItem::from_syntax(id, &parsed.arena)
                    .and_then(|article_item| {
                        let kind = article_item.identifier(&parsed.arena, input)?;
                        let id = article_item.item_id(&parsed.arena, input)?;

                        Some((kind.into(), id.into(), md.clone(), article_item))
                    })
            })
            .collect::<Vec<_>>()
    }).collect()
}

fn find_md(db: &dyn Renderer, lkind: String, lid: String) -> Option<(FileId, ArticleItem)> {
    let parsed = db.parse_all_mds();
    parsed.into_iter()
        .find(|(kind, id, _path, _item)| {
            &lkind == kind && &lid == id
        })
        .map(|(_kind, _id, path, item)| (path, item))
}

fn render_item(db: &dyn Renderer, kind: String, id: String) -> RenderResult {
    let article = db.find_md(kind.clone(), id.clone());

    let (path, article_item)
        = match article {
        Some(a) => a,
        None => return RenderResult {
            output: format!("Couldn't render, article {}:{} not found", kind, id),
            arena: Default::default()
        }
    };

    let input = db.input_md(path.clone());
    let mut parsed = db.parse_md_syntax(path);

    let rendered = _render(db, article_item, &mut parsed.arena, &input)
        .unwrap_or_else(|| "Couldn't render, found errors".to_string());

    RenderResult {
        output: rendered,
        arena: parsed.arena
    }
}
