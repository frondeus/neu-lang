use crate::result::RenderResult;
use neu_parser::{NodeId, Arena, Children};
use neu_syntax::Nodes;
use neu_eval::eval;

mod result;

fn _render(id: NodeId, nodes: &Arena, input: &str) -> Option<()> {
    let node = nodes.get(id);
    let mut children = Children::new(node.children.iter().copied(), nodes);
    let (_, article) = children.find_node(Nodes::ArticleItem)?;
    let mut children = Children::new(article.children.iter().copied(), nodes);
    let (strukt, _) = children.find_node(Nodes::Struct)?;
    let strukt = eval(strukt, nodes, input);

    let (_, body) = children.find_node(Nodes::ArticleBody)?;
    dbg!(&strukt);
    dbg!(&body);

    Some(())
}

pub fn render(id: NodeId, nodes: &Arena, input: &str) -> RenderResult {
    _render(id, nodes, input);
    RenderResult {
        output: "".to_string(),
        errors: vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use neu_parser::State;
    use neu_syntax::{lexers::article_item_file::Lexer,
                     parsers::article_item::parser};

    #[test]
    fn eval_tests() {
        test_runner::test_snapshots("md",
                                    "render",
                                    |input| {
            let lexer = Lexer::new(input);

            let res = State::parse(lexer, parser());
            let result = render(res.root, &res.nodes, input);

            result.display(input).to_string()
        })
            .unwrap();
    }
}
