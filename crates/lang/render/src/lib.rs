use crate::result::RenderResult;
use neu_parser::Arena;
use neu_eval::{eval, Value};
use std::fmt;
use neu_syntax::ast::ArticleItem;

mod result;

struct HtmlValue<'v> {
    value: &'v Value
}

impl<'v> fmt::Display for HtmlValue<'v> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width = f.width().unwrap_or_default();
        match self.value {
            Value::Number(n) => write!(f, "{}", n),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::String(s) => write!(f, "{}", s),
            Value::Array(a) => {
                write!(f, "[")?;
                if !a.is_empty() {
                    if let Some(v) = a.iter().next() {
                        write!(f, " {}", v)?;
                    }
                    for v in a.iter().skip(1) {
                        write!(f, ", {}", v)?;
                    }
                    write!(f, " ")?;
                }
                write!(f, "]")
            },
            Value::Struct(s) => {
                write!(f, "{{")?;
                if !s.is_empty() {
                    writeln!(f)?;
                }
                let c_width = width + 4;
                for (k, v) in s.iter() {
                    writeln!(f, "{:width$}{} = {:#width$},", " ", k, v, width = c_width)?;
                }
                write!(f, "{:width$}}}", " ", width = width)
            },
            Value::Lazy { .. } => unreachable!("lazy render")
        }
    }
}


fn render_value(value: &Value) -> HtmlValue {
    HtmlValue { value } 
}

fn _render(article_item: ArticleItem, nodes: &mut Arena, input: &str) -> Option<RenderResult> {
    let mut output = String::default();

    let article_item = article_item.anchor_body(nodes);

    let strukt_eval = eval(article_item.strukt?, nodes, input);
    let mut strukt = strukt_eval.value?.into_struct()?;

    let body_eval = eval(article_item.body?, nodes, input);
    let body = body_eval.value?;


    if let Some(title) = strukt.remove("title") {
        output.push_str(&format!("<h1>{}</h1>\n", render_value(&title)));
    }

    if !strukt.is_empty() {
        output.push_str("<table>");
        for (key, value) in strukt {
            output.push_str("<tr>");
            output.push_str(&format!("<th>{}</th>", key));
            output.push_str(&format!("<td>{}</td>", render_value(&value)));
            output.push_str("</tr>");
        }
        output.push_str("</table>");
    }

    output.push_str(&format!("{}", render_value(&body)));

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
            let article_item = ArticleItem::build(res.root, &res.arena);
            let result = render(article_item, &mut res.arena, input);

            result.display(input, &res.arena).to_string()
        })
            .unwrap();
    }
}
