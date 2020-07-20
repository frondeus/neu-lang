use crate::db::Renderer;
use crate::result::RenderResult;
use neu_eval::Value;
use neu_parser::{NodeId, ParseResult};
use neu_syntax::ast::{ArticleItem, ArticleRef, Ast};
use neu_syntax::db::FileId;
use neu_syntax::Nodes;
use std::collections::{BTreeMap, BTreeSet};

mod result;

mod html;

pub mod db;

fn eval(db: &dyn Renderer, file: FileId, id: NodeId, result: &mut RenderResult) -> Option<Value> {
    let evaled = db.eval(file, id);
    result.errors.merge(&evaled.errors);
    evaled.value.clone()
}

fn render_strukt(strukt: BTreeMap<String, Value>, result: &mut RenderResult) {
    if !strukt.is_empty() {
        result.output.push_str(r#"<table>"#);
        for (key, value) in strukt {
            result.output.push_str("<tr>");
            result.output.push_str(&format!(r#"<th class="align-right">{}</th>"#, key));
            result.output.push_str(&format!("<td>{}</td>", html::render_value(&value)));
            result.output.push_str("</tr>");
        }
        result.output.push_str("</table>\n");
    }
}

fn render_mentions(
    db: &dyn Renderer,
    kind: Option<&str>,
    id: Option<&str>,
    result: &mut RenderResult
) {
    if let (Some(kind), Some(id)) = (kind, id) {
        let mentions = db
            .all_mentions()
            .into_iter()
            .filter(|mention| mention.kind == kind)
            .filter(|mention| mention.id == id)
            .collect::<BTreeSet<_>>();

        if !mentions.is_empty() {
            result.output.push_str(r#"<table>"#);
            result.output.push_str("<tr>");
            result.output.push_str("<th>Mentioned in</th>");
            result.output.push_str("</tr>");

            for mention in mentions {
                let orig_item = db.find_md(mention.orig_kind.clone(), mention.orig_id.clone());
                result.output.push_str("<tr><td>");
                match orig_item {
                    Some((orig_path, orig_item)) => {
                        let title = orig_item
                            .strukt
                            .and_then(|strukt| {
                                let title = eval(db, orig_path, strukt, result)?
                                    .into_struct()?
                                    .remove("title")?;
                                Some(html::render_value(&title).to_string())
                            })
                            .unwrap_or_else(|| "???".into());

                        result.output.push_str(&format!(
                            r#"<a href="/{kind}/{id}">{title}</a>"#,
                            kind = mention.orig_kind,
                            id = mention.orig_id,
                            title = title
                        ));
                    }
                    None => {
                        result.output.push_str(&format!(
                            r#"<span class="error">Couldn't find {kind}:{id}</span>"#,
                            kind = mention.orig_kind,
                            id = mention.orig_id
                        ));
                    }
                }
                result.output.push_str("</td></tr>");
            }
            result.output.push_str("</table>\n");
        }
    }
}

fn render_body(
    db: &dyn Renderer,
    file_id: FileId,
    article_item: &ArticleItem,
    parsed: &ParseResult,
    result: &mut RenderResult
) {
    let arena = &parsed.arena;
    if let Some(body) = article_item.body {
        let body = arena
            .get(body)
            .children
            .iter()
            .copied()
            .collect::<Vec<NodeId>>();
        for body_id in body {
            let body = arena.get(body_id);
            if body.is(Nodes::Error) {
                let err = parsed.errors.get(body_id).expect("Error");
                let s = format!(r#"<div class="error">{}</div>"#, err);
                result.output.push_str(&s);
            } else if body.is(Nodes::Markdown) {
                if let Some(markdown) = eval(db, file_id, body_id, result) {
                    result.output.push_str(&format!("{}", html::render_value(&markdown)));
                }
            } else if body.is(Nodes::ArticleItem) {
                let input = db.input(file_id);
                let article_item =
                    ArticleItem::from_syntax(body_id, arena).expect("body is ArticleItem");
                let kind = article_item.identifier(arena, &input).unwrap_or("???");
                let id = article_item.item_id(arena, &input).unwrap_or("???");
                result.output.push_str(&format!(
                    r#"<div class="article-item" id="{}_{}" >"#,
                    kind, id
                ));
                _render(db, file_id, article_item, parsed, result);
                result.output.push_str("</div>\n");
            } else if body.is(Nodes::ArticleRef) {
                let input = db.input(file_id);
                let article_ref =
                    ArticleRef::from_syntax(body_id, arena).expect("body is ArticleRef");
                let kind = article_ref.identifier(arena, &input).unwrap_or("???");
                let id = article_ref.item_id(arena, &input).unwrap_or("???");
                result.output.push_str(&format!(
                    r#"<div class="article-item" id="{}_{}" >"#,
                    kind, id
                ));
                let rendered = db.render_item(kind.into(), id.into());
                result.output.push_str(&rendered.output);
                result.output.push_str("</div>\n");
            } else {
                let s = format!(r#"<div class="todo">{:?}</div>"#, body);
                result.output.push_str(&s);
            }
        }
    }
}

fn _render(
    db: &dyn Renderer,
    file_id: FileId,
    article_item: ArticleItem,
    parsed: &ParseResult,
    result: &mut RenderResult,
) {
    let input = db.input(file_id);
    let kind = article_item.identifier(&parsed.arena, &input);
    let id = article_item.item_id(&parsed.arena, &input);

    let mut strukt = article_item
        .strukt
        .and_then(|strukt| eval(db, file_id, strukt, result)?.into_struct())
        .unwrap_or_default();

    if let Some(title) = strukt.remove("title") {
        result
            .output
            .push_str(&format!("<h1>{}</h1>\n", html::render_value(&title)));
    }

    result.output.push_str(r#"<div class="side-table">"#);

    render_strukt(strukt, result);
    render_mentions(db, kind, id, result);

    result.output.push_str("</div>");

    render_body(db, file_id, &article_item, parsed, result);
}

#[cfg(test)]
mod tests {
    use super::*;
    use neu_syntax::db::{FileKind, Parser};
    use std::sync::Arc;

    #[salsa::database(
        crate::db::RendererDatabase,
        neu_eval::db::EvaluatorDatabase,
        neu_analyze::db::AnalyzerDatabase,
        neu_syntax::db::ParserDatabase
    )]
    #[derive(Default)]
    struct TestDb {
        storage: salsa::Storage<Self>,
    }

    impl salsa::Database for TestDb {}

    #[test]
    fn render_tests() {
        test_runner::test_snapshots("md", "render", |input| {
            let mut db = TestDb::default();
            let path = db.file_id(("test".into(), FileKind::Md));
            db.set_all_mds(Arc::new(Some(path.clone()).into_iter().collect()));
            db.set_input(path.clone(), Arc::new(input.into()));
            let result = db.render_md(path);

            result.display(input).to_string()
        })
        .unwrap();
    }
}
