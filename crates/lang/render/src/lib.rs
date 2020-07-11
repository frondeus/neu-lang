use crate::result::RenderResult;
use neu_parser::{Arena, Children, NodeId};
use neu_eval::{eval, Value};
use neu_syntax::ast::{ArticleItem, ArticleRef};
use neu_syntax::Nodes;

mod result;

mod html;

mod db;

fn _render(article_item: ArticleItem, nodes: &mut Arena, input: &str) -> Option<RenderResult> {
    let mut output = String::default();

    article_item.anchor_body(nodes);


    let strukt_eval = eval(article_item.strukt?, nodes, input);
    let mut strukt = strukt_eval.value?.into_struct()?;


    if let Some(title) = strukt.remove("title") {
        output.push_str(&format!("<h1>{}</h1>\n", html::render_value(&title)));
    }

    if !strukt.is_empty() {
        output.push_str(r#"<table class="side-table">"#);
        for (key, value) in strukt {
            output.push_str("<tr>");
            output.push_str(&format!("<th>{}</th>", key));
            output.push_str(&format!("<td>{}</td>", html::render_value(&value)));
            output.push_str("</tr>");
        }
        output.push_str("</table>");
    }

    let body = article_item.body?;
    let body = nodes.get(body).children.iter().copied().collect::<Vec<NodeId>>();
    for body_id in body {
        let body = nodes.get(body_id);
        if body.is(Nodes::Markdown) {
            let markdown_eval = eval(body_id, nodes, input);
            let markdown = markdown_eval.value?;
            output.push_str(&format!("{}", html::render_value(&markdown)));
        }
        else if body.is(Nodes::ArticleItem) {
            output.push_str(r#"<div class="article-item">"#);
            let article_item = ArticleItem::build(body_id, nodes);
            let rendered = _render(article_item, nodes, input)?;
            output.push_str(rendered.output.as_str());
            output.push_str("</div>");
        }
        else if body.is(Nodes::ArticleRef) {
            //TODO:
            let article_ref = ArticleRef::build(body_id, nodes);
            let kind = article_ref.identifier(nodes, input).unwrap_or("???");
            let id = article_ref.item_id(nodes, input).unwrap_or("???");
            let s = format!(r#"<div class="todo"><a href="/{}/{}">{:?}</a></div>"#, kind, id, body);
            output.push_str(&s);
        }
        else {
            let s = format!(r#"<div class="todo">{:?}</div>"#, body);
            output.push_str(&s);
        }
    }

    Some(RenderResult {
        output
    })
}


pub fn render(article_item: ArticleItem, nodes: &mut Arena, input: &str) -> RenderResult {
    _render(article_item, nodes, input).unwrap_or_else(|| {
        RenderResult {
            output: "Couldn't render, found errors".to_string()
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use neu_parser::State;
    use neu_syntax::{lexers::article_item_file::Lexer,
                     parsers::article_item::parser};

    #[test]
    fn render_tests() {
        test_runner::test_snapshots("md",
                                    "render",
                                    |input| {
            let lexer = Lexer::new(input);

            let mut res = State::parse(lexer, parser());
            let article_item = ArticleItem::from_root(res.root, &res.arena);
            let result = render(article_item, &mut res.arena, input);

            result.display(input, &res.arena).to_string()
        })
            .unwrap();
    }
}
