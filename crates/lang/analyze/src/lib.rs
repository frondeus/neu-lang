use crate::db::Analyzer;
use neu_parser::{Arena, Children, Node, NodeId};
use neu_syntax::ast::{ArticleItem, ArticleRef, Ast};
use neu_syntax::Nodes;
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

fn find_mentions_in_md(
    _db: &dyn Analyzer,
    node: &Node,
    nodes: &Arena,
    input: &str,
    orig_kind: &str,
    orig_id: &str,
    mentions: &mut Vec<Mention>,
) {
    let children = node.children.iter().copied().collect::<Vec<NodeId>>();
    for child_id in children {
        let child = nodes.get(child_id);
        if child.is(Nodes::Md_Link) {
            let mut children = Children::new(child.children.iter().copied(), nodes);
            if let Some((_, url)) = children.find_node(Nodes::Md_LinkUrl) {
                let text = &input[url.span];
                //TODO Parse :
                let link_regex = Regex::new("([a-z_A-Z0-9]+):([0-9A-Fa-f]{8})").expect("Regex");
                if let Some(cap) = link_regex.captures(text) {
                    let kind = cap.get(1).expect("G1").as_str();
                    let id = cap.get(2).expect("G2").as_str();
                    mentions.push(Mention::new(orig_kind, orig_id, kind, id));
                }
            }
        }
        find_mentions_in_md(_db, child, nodes, input, orig_kind, orig_id, mentions);
    }
}

fn find_mentions(
    _db: &dyn Analyzer,
    article_item: ArticleItem,
    nodes: &mut Arena,
    input: &str,
    mentions: &mut Vec<Mention>,
) -> Option<()> {
    let orig_kind = article_item.identifier(nodes, input).unwrap_or("???");
    let orig_id = article_item.item_id(nodes, input).unwrap_or("???");

    let body = article_item.body?;
    let body = nodes
        .get(body)
        .children
        .iter()
        .copied()
        .collect::<Vec<NodeId>>();

    for body_id in body {
        let body = nodes.get(body_id);
        if body.is(Nodes::ArticleItem) {
            let article_item =
                ArticleItem::from_syntax(body_id, nodes).expect("body is ArticleItem");
            let kind = article_item.identifier(nodes, input).unwrap_or("???");
            let id = article_item.item_id(nodes, input).unwrap_or("???");
            mentions.push(Mention::new(orig_kind, orig_id, kind, id));
        } else if body.is(Nodes::ArticleRef) {
            let article_item = ArticleRef::from_syntax(body_id, nodes).expect("body is ArticleRef");
            let kind = article_item.identifier(nodes, input).unwrap_or("???");
            let id = article_item.item_id(nodes, input).unwrap_or("???");
            mentions.push(Mention::new(orig_kind, orig_id, kind, id));
        } else if body.is(Nodes::Markdown) {
            find_mentions_in_md(_db, body, nodes, input, orig_kind, orig_id, mentions);
        }
    }

    Some(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    use neu_syntax::db::{FileId, FileKind, Parser};
    use std::fmt;

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
            let path: FileId = ("test".into(), FileKind::Md);
            db.set_all_mds(Some(path.clone()).into_iter().collect());
            db.set_input(path, input.into());

            db.all_mentions()
                .into_iter()
                .map(|mention| mention.to_string())
                .join("\n")
        })
        .unwrap();
    }
}
