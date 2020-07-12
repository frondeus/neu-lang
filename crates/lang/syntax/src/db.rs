use crate::ast::{ArticleItem, Ast};
use neu_parser::{NodeId, ParseResult, State};
use std::collections::HashSet;

pub type FileId = String;
pub type Kind = String;
pub type ArticleId = String;

#[salsa::query_group(ParserDatabase)]
pub trait Parser: salsa::Database {
    #[salsa::input]
    fn input_md(&self, path: FileId) -> String;
    #[salsa::input]
    fn input_neu(&self, path: FileId) -> String;

    #[salsa::input]
    fn all_mds(&self) -> HashSet<FileId>;
    #[salsa::input]
    fn all_neu(&self) -> HashSet<FileId>;

    fn parse_md_syntax(&self, path: FileId) -> ParseResult;
    fn parse_all_mds(&self) -> Vec<(Kind, ArticleId, FileId, ArticleItem)>;
    fn find_md(&self, kind: Kind, id: ArticleId) -> Option<(FileId, ArticleItem)>;

    fn parse_neu_syntax(&self, path: FileId) -> ParseResult;
    fn parse_all_neu(&self) -> Vec<(FileId, NodeId)>;
}

fn parse_neu_syntax(db: &dyn Parser, path: FileId) -> ParseResult {
    use crate::lexers::neu::Lexer;
    use crate::parsers::neu::parser;

    let input = db.input_neu(path);
    let input = input.as_str();
    let lexer = Lexer::new(&input);
    State::parse(lexer, parser())
}

fn parse_md_syntax(db: &dyn Parser, path: FileId) -> ParseResult {
    use crate::lexers::article_item_file::Lexer;
    use crate::parsers::article_item::parser;

    let input = db.input_md(path);
    let input = input.as_str();
    let lexer = Lexer::new(&input);
    State::parse(lexer, parser())
}

fn parse_all_neu(db: &dyn Parser) -> Vec<(FileId, NodeId)> {
    db.all_neu()
        .into_iter()
        .map(|path| {
            let parsed = db.parse_neu_syntax(path.clone());

            (path, parsed.root)
        })
        .collect()
}

fn parse_all_mds(db: &dyn Parser) -> Vec<(Kind, ArticleId, FileId, ArticleItem)> {
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

fn find_md(db: &dyn Parser, lkind: Kind, lid: ArticleId) -> Option<(FileId, ArticleItem)> {
    let parsed = db.parse_all_mds();
    parsed
        .into_iter()
        .find(|(kind, id, _path, _item)| &lkind == kind && &lid == id)
        .map(|(_kind, _id, path, item)| (path, item))
}
