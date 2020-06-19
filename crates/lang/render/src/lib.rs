use crate::result::RenderResult;
use neu_parser::{NodeId, Arena, Children};
use neu_syntax::Nodes;
use neu_eval::{eval, Value};
use std::fmt;

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

fn _render(id: NodeId, nodes: &mut Arena, input: &str) -> Option<RenderResult> {
    let mut output = String::default();
    let node = nodes.get(id);
    let mut children = Children::new(node.children.iter().copied(), nodes);
    let (_, article) = children.find_node(Nodes::ArticleItem)?;
    let mut children = Children::new(article.children.iter().copied(), nodes);
    let (strukt, _) = children.find_node(Nodes::Struct)?;

    let (_, body) = children.find_node(Nodes::ArticleBody)?;
    let mut body = body.clone();

    body.parent = Some(strukt);
    let body = nodes.add(body);

    let body_eval = eval(body, nodes, input);
    let body = body_eval.value?;

    output.push_str(&format!("{}", render_value(&body)));

    Some(RenderResult {
        output,
        errors: vec![]
    })
}


pub fn render(id: NodeId, nodes: &mut Arena, input: &str) -> RenderResult {
    _render(id, nodes, input).unwrap_or_else(|| {
        RenderResult {
            output: "Something went very wrong".to_string(),
            errors: vec![]
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
            let result = render(res.root, &mut res.arena, input);

            result.display(input).to_string()
        })
            .unwrap();
    }
}
