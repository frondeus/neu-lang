use neu_parser::core::{NodeId, Arena, Node, Name};
use neu_parser::Nodes;

#[derive(Debug)]
pub enum Value {
    Number(i64),
    Boolean(bool),
    String(String),
}

struct Children<'a, I> {
    iter: I,
    arena: &'a Arena
}

impl<'a, I> Iterator for Children<'a, I>
    where I: Iterator<Item = NodeId>
{
    type Item = (NodeId, &'a Node);

    fn next(&mut self) -> Option<Self::Item> {
        let mut id = self.iter.next()?;
        let mut node = self.arena.get(id);
        while node.is(Nodes::Trivia) {
            id = self.iter.next()?;
            node = self.arena.get(id);
        }

        Some((id, node))
    }
}


impl<'a, I> Children<'a, I>
where I: Iterator<Item = NodeId>
{
    fn new(iter: I, arena: &'a Arena) -> Self {
        Self { iter, arena }
    }

    fn find_node(&mut self, expected: Name) -> Option<(NodeId, &'a Node)> {
        let mut next = self.next()?;
        while !next.1.is(expected) {
            next = self.next()?;
        }
        Some(next)
    }
}

struct Eval<'a> {
    pub nodes: &'a Arena,
    pub input: &'a str
}

impl<'a> Eval<'a> {
    fn eval(&self, id: NodeId) -> Option<Value> {
        let node = self.nodes.get(id);

        if node.is(Nodes::Root) {
            return node.children.iter()
                .filter_map(|child| self.eval(*child))
                .next();
        }
        if !node.is(Nodes::Value) { return None; }

        let mut children = Children::new(node.children.iter().copied(), self.nodes);

        let text = &self.input[node.span];

        if node.is(Nodes::Number) { return Some(Value::Number(text.parse().unwrap())); }

        if node.is(Nodes::Boolean) { return Some(Value::Boolean(text == "true")); }

        if node.is(Nodes::String) {
            let len = text.len();
            let text = &text[1..=len - 2];
            return Some(Value::String(text.into()));
        }

        if node.is(Nodes::Unary) {
            let (_, op) = children.find_node(Nodes::Op)?;
            let (value, _) = children.find_node(Nodes::Value)?;
            let value = self.eval(value)?;
            let text_op = &self.input[op.span];
            return match (text_op, value) {
                ("-", Value::Number(i))  => Some(Value::Number(-i)),
                ("!", Value::Boolean(b))  => Some(Value::Boolean(!b)),
                _ => None
            };
        }

        if node.is(Nodes::Binary) {
            let (left, _) = children.find_node(Nodes::Value)?;
            let left = self.eval(left)?;
            let (_, op) = children.find_node(Nodes::Op)?;
            let (right, _) = children.find_node(Nodes::Value)?;
            let right = self.eval(right)?;
            let text_op = &self.input[op.span];
            return match (left, text_op, right) {
                (Value::Number(l), "-", Value::Number(r))  => Some(Value::Number(l - r)),
                (Value::Number(l), "+", Value::Number(r))  => Some(Value::Number(l + r)),
                (Value::Number(l), "*", Value::Number(r))  => Some(Value::Number(l * r)),
                (Value::Number(l), "/", Value::Number(r))  => Some(Value::Number(l / r)),
                (Value::Boolean(l), "==", Value::Boolean(r))  => Some(Value::Boolean(l == r)),
                _ => None
            };
        }

        if node.is(Nodes::Parens) {
            let (value, _) = children.find_node(Nodes::Value)?;
            return self.eval(value);
        }

        None
    }
}

pub fn eval(id: NodeId, nodes: &Arena, input: &str) -> Option<Value> {
    Eval { nodes, input }.eval(id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
    use neu_parser::core::{Lexer, State};
    use neu_parser::parser;

    #[test_case("4", "number")]
    #[test_case("    5", "skip_trivia")]
    #[test_case("true", "bool_t")]
    #[test_case("false", "bool_f")]
    #[test_case("-5", "unary_int")]
    #[test_case("!true", "unary_bool")]
    #[test_case("4 + 2", "binary_int")]
    #[test_case("true == false", "binary_bool")]
    #[test_case("4 + 2 * 5", "pratt_int")]
    #[test_case("(4 + 2) * 5", "parens")]
    #[test_case("true ==", "binary_error")]
    #[test_case(r#" "foo" "#, "string")]
    #[test_case("???", "error")]
    fn tests(input: &str, test_case_name: &str) {
        let lexer = Lexer::new(input);

        let res = State::parse(lexer, parser());
        let result = eval(res.root, &res.nodes, input);
        println!("{}", res.display(input));
        neu_parser::core::testing::snap(
            format!("```\n{}\n```\n\n{:#?}", input, result),
            file!(),
            &format!("eval_{}", test_case_name),
        );
    }

}
