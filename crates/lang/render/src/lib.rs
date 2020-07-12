use crate::db::Renderer;
use neu_eval::eval;
use neu_parser::{Arena, NodeId};
use neu_syntax::ast::{ArticleItem, ArticleRef, Ast};
use neu_syntax::Nodes;
use std::collections::BTreeSet;

mod result;

mod html;

pub mod db;

fn _render(
    db: &dyn Renderer,
    article_item: ArticleItem,
    nodes: &mut Arena,
    input: &str,
) -> Option<String> {
    let mut output = String::default();

    article_item.anchor_body(nodes);

    let kind = article_item.identifier(nodes, input).unwrap_or("???");
    let id = article_item.item_id(nodes, input).unwrap_or("???");

    let strukt_eval = eval(article_item.strukt?, nodes, input);
    let mut strukt = strukt_eval.value?.into_struct()?;

    if let Some(title) = strukt.remove("title") {
        output.push_str(&format!("<h1>{}</h1>\n", html::render_value(&title)));
    }

    output.push_str(r#"<div class="side-table">"#);

    if !strukt.is_empty() {
        output.push_str(r#"<table>"#);
        for (key, value) in strukt {
            output.push_str("<tr>");
            output.push_str(&format!(r#"<th class="align-right">{}</th>"#, key));
            output.push_str(&format!("<td>{}</td>", html::render_value(&value)));
            output.push_str("</tr>");
        }
        output.push_str("</table>\n");
    }

    let mentions = db.find_mentions().into_iter()
            .filter(|mention| mention.kind == kind)
            .filter(|mention| mention.id == id)
            .collect::<BTreeSet<_>>();

    if !mentions.is_empty() {
        output.push_str(r#"<table>"#);
        output.push_str("<tr>");
        output.push_str("<th>Mentioned in</th>");
        output.push_str("</tr>");

        for mention in mentions {
            let orig_item = db.find_md(mention.orig_kind.clone(), mention.orig_id.clone());
            output.push_str("<tr><td>");
            match orig_item {
                Some((orig_path, orig_item)) => {
                    let orig_input = db.input_md(orig_path.clone());
                    let mut orig_parsed = db.parse_md_syntax(orig_path.clone());
                    let strukt_eval = eval(orig_item.strukt?, &mut orig_parsed.arena, &orig_input);
                    let mut strukt = strukt_eval.value?.into_struct()?;

                    let title = strukt.remove("title")
                        .map(|title| html::render_value(&title).to_string())
                        .unwrap_or_else(|| "???".into());

                    output.push_str(&format!(r#"<a href="/{kind}/{id}">{title}</a>"#,
                        kind = mention.orig_kind,
                        id = mention.orig_id,
                        title = title
                    ));
                },
                None => {
                    output.push_str(&format!(r#"<span class="error">Couldn't find {kind}:{id}</span>"#,
                         kind = mention.orig_kind,
                         id = mention.orig_id
                    ));
                }
            }
            output.push_str("</td></tr>");
        }
        output.push_str("</table>\n");
    }

    output.push_str("</div>");

    let body = article_item.body?;
    let body = nodes
        .get(body)
        .children
        .iter()
        .copied()
        .collect::<Vec<NodeId>>();
    for body_id in body {
        let body = nodes.get(body_id);
        if body.is(Nodes::Error) {
            let err = nodes.component(body_id).expect("Error");
            let s = format!(r#"<div class="error">{}</div>"#, err);
            output.push_str(&s);
        } else if body.is(Nodes::Markdown) {
            let markdown_eval = eval(body_id, nodes, input);
            let markdown = markdown_eval.value?;
            output.push_str(&format!("{}", html::render_value(&markdown)));
        } else if body.is(Nodes::ArticleItem) {
            let article_item =
                ArticleItem::from_syntax(body_id, nodes).expect("body is ArticleItem");
            let kind = article_item.identifier(nodes, input).unwrap_or("???");
            let id = article_item.item_id(nodes, input).unwrap_or("???");
            output.push_str(&format!(
                r#"<div class="article-item" id="{}_{}" >"#,
                kind, id
            ));
            let rendered = _render(db, article_item, nodes, input)?;
            output.push_str(rendered.as_str());
            output.push_str("</div>\n");
        } else if body.is(Nodes::ArticleRef) {
            let article_ref = ArticleRef::from_syntax(body_id, nodes).expect("body is ArticleRef");
            let kind = article_ref.identifier(nodes, input).unwrap_or("???");
            let id = article_ref.item_id(nodes, input).unwrap_or("???");
            output.push_str(&format!(
                r#"<div class="article-item" id="{}_{}" >"#,
                kind, id
            ));
            let rendered = db.render_item(kind.into(), id.into());
            output.push_str(&rendered.output);
            output.push_str("</div>\n");
        } else {
            let s = format!(r#"<div class="todo">{:?}</div>"#, body);
            output.push_str(&s);
        }
    }

    Some(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use neu_syntax::db::Parser;

    #[salsa::database(crate::db::RendererDatabase,
    neu_analyze::db::AnalyzerDatabase,
    neu_syntax::db::ParserDatabase)]
    #[derive(Default)]
    struct TestDb {
        storage: salsa::Storage<Self>,
    }

    impl salsa::Database for TestDb {}

    #[test]
    fn render_tests() {
        test_runner::test_snapshots("md", "render", |input| {
            let mut db = TestDb::default();
            let path: String = "test".into();
            db.set_all_mds(Some(path.clone()).into_iter().collect());
            db.set_input_md(path.clone(), input.into());
            let result = db.render_md(path);

            result.display(input).to_string()
        })
        .unwrap();
    }
}
