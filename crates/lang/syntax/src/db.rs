use crate::ast::{ArticleItem, MainArticle, Value};
use neu_canceled::Canceled;
use microtree_parser::{GreenSink, ParseResult, State};
use microtree::{Ast, Red};
use std::collections::HashSet;
use std::sync::Arc;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum FileKind {
    Md,
    Neu,
}
//pub type FileId = (String, FileKind);
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct FileId(salsa::InternId);
impl salsa::InternKey for FileId {
    fn from_intern_id(id: salsa::InternId) -> Self {
        Self(id)
    }

    fn as_intern_id(&self) -> salsa::InternId {
        self.0
    }
}

pub type FileIdData = (String, FileKind);
pub type Kind = String;
pub type ArticleId = String;

#[salsa::query_group(ParserDatabase)]
pub trait Parser: salsa::Database {
    #[salsa::interned]
    fn file_id(&self, data: FileIdData) -> FileId;

    #[salsa::input]
    fn input(&self, path: FileId) -> Arc<String>;

    #[salsa::input]
    fn all_mds(&self) -> Arc<HashSet<FileId>>;
    #[salsa::input]
    fn all_neu(&self) -> Arc<HashSet<FileId>>;

    fn parse_syntax(&self, path: FileId) -> Arc<ParseResult>;
    fn parse_md_syntax(&self, path: FileId) -> Arc<ParseResult>;
    fn parse_all_mds(&self) -> Vec<(Kind, ArticleId, FileId, ArticleItem)>;
    fn find_md(&self, kind: Kind, id: ArticleId) -> Option<(FileId, ArticleItem)>;

    fn parse_neu_syntax(&self, path: FileId) -> Arc<ParseResult>;
    fn parse_all_neu(&self) -> Vec<(FileId, Value)>;
}

fn parse_syntax(db: &dyn Parser, file: FileId) -> Arc<ParseResult> {
    Canceled::cancel_if(db.salsa_runtime());
    let file_data = db.lookup_file_id(file);
    //dbg!(&file_data);
    match file_data.1 {
        FileKind::Md => db.parse_md_syntax(file),
        FileKind::Neu => db.parse_neu_syntax(file),
    }
}

fn parse_neu_syntax(db: &dyn Parser, path: FileId) -> Arc<ParseResult> {
    Canceled::cancel_if(db.salsa_runtime());
    use crate::lexers::neu::Lexer;
    use crate::parsers::neu::parser;

    let input = db.input(path);
    let input = input.as_str();
    let lexer = Lexer::new(input);
    let sink: GreenSink = State::parse(lexer, parser());
    Arc::new(sink.finish())
}

fn parse_md_syntax(db: &dyn Parser, path: FileId) -> Arc<ParseResult> {
    Canceled::cancel_if(db.salsa_runtime());
    use crate::lexers::article_item_file::Lexer;
    use crate::parsers::article_item::parser;

    let input = db.input(path);
    let input = input.as_str();
    //dbg!(&input);

    //let _dbg_sink: DbgSink = State::parse(Lexer::new(input), parser());
    let sink: GreenSink = State::parse(Lexer::new(input), parser());
    Arc::new(sink.finish())
}

fn parse_all_neu(db: &dyn Parser) -> Vec<(FileId, Value)> {
    Canceled::cancel_if(db.salsa_runtime());
    db.all_neu()
        .iter()
        .filter_map(|path| {
            let parsed = db.parse_neu_syntax(*path);
            let red = Red::root(parsed.root.clone());

            Value::new(red).map(|ast| (*path, ast))
        })
        .collect()
}

fn parse_all_mds(db: &dyn Parser) -> Vec<(Kind, ArticleId, FileId, ArticleItem)> {
    Canceled::cancel_if(db.salsa_runtime());
    db.all_mds()
        .iter()
        .filter_map(|md| {
            let parsed = db.parse_syntax(*md);
            let red = Red::root(parsed.root.clone());
            let main = MainArticle::new(red)?;
            Some((md, main))
        })
        .flat_map(|(md, main_article)| {
            let items = main_article
                        .get_articles()
                        .flat_map(|sub_article| sub_article.get_articles())
                        .map(|sub_article| ArticleItem::from(sub_article))
                        .map(move |item| (md, item));

            let item: ArticleItem = main_article.into();

            Some((md, item)).into_iter().chain(items)
        })
        .filter_map(|(md, article_item)| {
            //dbg!(&article_item);
            let kind = article_item
                .item_ident()?
                .red()
                .to_string();
            //dbg!(&kind);

            let id = article_item
                .item_id()?
                .red()
                .to_string();

            //dbg!(&id);

            Some((kind, id, *md, article_item))
        })
        .collect()
}

fn find_md(db: &dyn Parser, lkind: Kind, lid: ArticleId) -> Option<(FileId, ArticleItem)> {
    Canceled::cancel_if(db.salsa_runtime());
    let parsed = db.parse_all_mds();
    parsed
        .into_iter()
        .find(|(kind, id, _path, _item)| &lkind == kind && &lid == id)
        .map(|(_kind, _id, path, item)| (path, item))
}
