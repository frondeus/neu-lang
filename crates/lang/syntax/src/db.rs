use crate::ast::{ArticleItem, Ast};
use crate::lexers::article_item_file::Lexer;
use crate::parsers::article_item::parser;
use neu_parser::{ParseResult, State};
use std::collections::HashSet;

pub type FileId = String;

#[salsa::query_group(ParserDatabase)]
pub trait Parser: salsa::Database {
    #[salsa::input]
    fn input_md(&self, path: FileId) -> String;

    #[salsa::input]
    fn all_mds(&self) -> HashSet<FileId>;

    fn parse_md_syntax(&self, path: FileId) -> ParseResult;
    // kind, id,    path,    ast
    fn parse_all_mds(&self) -> Vec<(String, String, FileId, ArticleItem)>;
    fn find_md(&self, kind: String, id: String) -> Option<(FileId, ArticleItem)>;
}

fn parse_md_syntax(db: &dyn Parser, path: FileId) -> ParseResult {
    let input = db.input_md(path);
    let input = input.as_str();
    let lexer = Lexer::new(&input);
    State::parse(lexer, parser())
}

fn parse_all_mds(db: &dyn Parser) -> Vec<(String, String, FileId, ArticleItem)> {
    db.all_mds()
        .into_iter()
        .flat_map(|md| {
            let input = db.input_md(md.clone());
            let input = input.as_str();
            let parsed = db.parse_md_syntax(md.clone());

            parsed
                .arena
                .enumerate()
                .filter_map(|(id, _)| {
                    ArticleItem::from_syntax(id, &parsed.arena).and_then(|article_item| {
                        let kind = article_item.identifier(&parsed.arena, input)?;
                        let id = article_item.item_id(&parsed.arena, input)?;

                        Some((kind.into(), id.into(), md.clone(), article_item))
                    })
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn find_md(db: &dyn Parser, lkind: String, lid: String) -> Option<(FileId, ArticleItem)> {
    let parsed = db.parse_all_mds();
    parsed
        .into_iter()
        .find(|(kind, id, _path, _item)| &lkind == kind && &lid == id)
        .map(|(_kind, _id, path, item)| (path, item))
}
