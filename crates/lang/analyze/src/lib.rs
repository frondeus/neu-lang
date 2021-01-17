use neu_syntax::ast::{ArticleItem, ArticleRef, SubArticle, Markdown};
use neu_syntax::reexport::Ast;
use regex::Regex;

pub mod db;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Mention {
    pub orig_kind: String,
    pub orig_id: String,
    pub kind: String,
    pub id: String,
}
impl Mention {
    pub fn new(
        orig_kind: impl Into<String>,
        orig_id: impl Into<String>,
        kind: impl Into<String>,
        id: impl Into<String>,
    ) -> Self {
        Self {
            orig_kind: orig_kind.into(),
            orig_id: orig_id.into(),
            kind: kind.into(),
            id: id.into(),
        }
    }
}

pub(crate) fn find_mentions(
    article_item: ArticleItem,
    mentions: &mut Vec<Mention>,
) -> Option<()> {
    let orig_kind = article_item.item_ident()?.red().to_string();
    let orig_id = article_item.item_id()?.red().to_string();
    let body = article_item.body()?;
    let red = body.red();
    let items = red
        .pre_order()
        .flat_map(|item| {
            if let Some(sub) = SubArticle::new(item.clone()) {
                sub_mention(sub, &orig_kind, &orig_id).into_iter()
                    .collect::<Vec<_>>()
            }
            else if let Some(re) = ArticleRef::new(item.clone()) {
                ref_mention(re, &orig_kind, &orig_id).into_iter()
                    .collect::<Vec<_>>()
            }
            else if let Some(md) = Markdown::new(item) {
                find_mentions_in_md(md, &orig_kind, &orig_id)
            }
            else { vec![] }
        });

    mentions.extend(items);

    Some(())
}

fn sub_mention(sub: SubArticle, orig_kind: &String, orig_id: &String) -> Option<Mention> {
    let header = sub.sub_article_header()?;
    let kind = header.item_ident_token()?.red().to_string();
    let id   = header.article_item_id_token()?.red().to_string();

    Some(Mention::new(orig_kind.clone(), orig_id.clone(), kind, id))
}

fn ref_mention(re: ArticleRef, orig_kind: &String, orig_id: &String) -> Option<Mention> {
    let kind = re.item_ident_token()?.red().to_string();
    let id = re.article_item_id_token()?.red().to_string();
    Some(Mention::new(orig_kind.clone(), orig_id.clone(), kind, id))
}

fn find_mentions_in_md<'a>(
    markdown: Markdown,
    orig_kind: &'a String,
    orig_id: &'a String,
) -> Vec<Mention> {
    lazy_static::lazy_static! {
        static ref LINK_REG: Regex = Regex::new(r"([a-z_A-Z0-9]+):([0-9A-Fa-f]{8})").expect("Regex");
    }
    markdown.all_links()
        .filter_map(|mdlink| mdlink.md_link_url_token())
        .filter_map(move |url| {
            match LINK_REG.captures(&url.red().to_string()) {
                Some(cap) => {
                    let kind = cap.get(1).unwrap().as_str();
                    let id = cap.get(2).unwrap().as_str();
                    Some(Mention::new(orig_kind.clone(), orig_id.clone(), kind, id))
                },
                None => None
            }
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    use neu_syntax::db::{FileKind, Parser};
    use std::fmt;
    use std::sync::Arc;
    use crate::db::Analyzer;

    impl fmt::Display for Mention {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "{}:{} in {}:{}",
                self.kind, self.id, self.orig_kind, self.orig_id
            )
        }
    }

    #[salsa::database(neu_syntax::db::ParserDatabase, crate::db::AnalyzerDatabase)]
    #[derive(Default)]
    struct TestDb {
        storage: salsa::Storage<Self>,
    }

    impl salsa::Database for TestDb {}

    #[test]
    fn analyze_tests() {
        test_runner::test_snapshots("md", "mentions", |input| {
            let mut db = TestDb::default();
            let path = db.file_id(("test".into(), FileKind::Md));
            db.set_all_mds(Arc::new(Some(path.clone()).into_iter().collect()));
            db.set_input(path, Arc::new(input.into()));

            db.all_mentions()
                .into_iter()
                .map(|mention| mention.to_string())
                .join("\n")
        })
        .unwrap();
    }
}
