use crate::db::Renderer;
use crate::result::RenderResult;
use neu_eval::Value;
use neu_syntax::{ast::{ArticleItem, ArticleRef, Markdown}, reexport::{Ast, Order, Red, SmolStr}};
use std::collections::{BTreeMap, BTreeSet};
use itertools::Itertools;

mod result;

mod html;

pub mod db;

fn eval(db: &dyn Renderer, red: Red, result: &mut RenderResult) -> Option<Value> {
    let evaled = db.eval(red);
    result.errors.merge(&evaled.errors);
    evaled.value.clone()
}

fn render_strukt(strukt: BTreeMap<SmolStr, Value>, result: &mut RenderResult) {
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

/*
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
*/

fn render_body(
    db: &dyn Renderer,
    article_item: ArticleItem,
    result: &mut RenderResult
) {
    if let Some(body) = article_item.body() {
        let red = body.red();
        if let Some(markdown) = eval(db, red, result) {
            result.output.push_str(&format!("{}", html::render_value(&markdown)));
        }
    }
}

fn _render(
    db: &dyn Renderer,
    article_item: ArticleItem,
    result: &mut RenderResult,
) {
    let kind = article_item.item_ident().map(|i| i.red().to_string()).unwrap_or_else(|| "???".to_string());
    let id = article_item.item_id().map(|i| i.red().to_string()).unwrap_or_else(|| "???".to_string());

    let mut strukt = article_item
        .strukt()
        .and_then(|strukt| eval(db, strukt.red(), result)?.into_struct())
        .unwrap_or_default();

    if let Some(title) = strukt.remove("title") {
        result
            .output
            .push_str(&format!("<h1>{}</h1>\n", html::render_value(&title)));
    }

    result.output.push_str(r#"<div class="side-table">"#);

    render_strukt(strukt, result);
    //render_mentions(db, kind, id, result);

    result.output.push_str("</div> \n");

    render_body(db, article_item, result);
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

            result.to_string()
        })
        .unwrap();
    }
}
