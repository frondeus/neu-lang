mod children;
mod value;
mod error;
mod result;
mod markdown;

use neu_parser::{NodeId, Arena, Node};
use neu_syntax::Nodes;
use value::Value;
use children::Children;
use error::Error;
use std::collections::BTreeMap;
use crate::result::EvalResult;

struct Eval<'a> {
    pub nodes: &'a Arena,
    pub input: &'a str,
    pub errors: Vec<(NodeId, Error)>
}

impl<'a> Eval<'a> {
    pub fn new(nodes: &'a Arena, input: &'a str) -> Self {
        Self {
            nodes,
            input,
            errors: vec![]
        }
    }

    #[allow(clippy::wrong_self_convention)]
    fn into_eager(&mut self, value: Value, recursive: bool) -> Option<Value> {
        match value {
            Value::Lazy { id } => {
                let v = self.eval(id)?;
                if !recursive { return Some(v); }
                self.into_eager(v, recursive)
            },
            Value::Struct(s) => {
                let s = s.into_iter()
                    .map(|(k, v)| {
                        let v = self.into_eager(v, recursive);
                        v.map(|v| (k, v))
                    }).collect::<Option<BTreeMap<String, Value>>>()?;
                Some(Value::Struct(s))
            },
            Value::Array(a) => {
                let a = a.into_iter()
                    .map(|v| {
                        self.into_eager(v, recursive)
                    }).collect::<Option<Vec<Value>>>()?;
                Some(Value::Array(a))
            },
            v => Some(v)
        }
    }

    fn eager_eval(&mut self, id: NodeId, recursive: bool) -> Option<Value> {
        let v = self.eval(id)?;
        self.into_eager(v, recursive)
    }

    fn expect_some<V>(&mut self, id: NodeId, v: Option<V>, error: Error) -> Option<V> {
        match v {
            Some(v) => Some(v),
            None => {
                self.errors.push((id, error));
                None
            }
        }
    }

    fn eval_identifier(&mut self, id: NodeId, node: &Node)  -> Option<Value> {
        let text = &self.input[node.span];
        let top = self.nodes.ancestors(id)
            .filter(|ancestor| {
                self.nodes.get(ancestor).is(Nodes::Struct)
            })
            .last();
        let top = self.expect_some(id, top, Error::ContextNotFound)?;
        let top = self.eval(top)?;
        let mut map = self.expect_some(id, top.into_struct(), Error::ValueNotStruct)?;
        self.expect_some(id, map.remove(text), Error::FieldNotFound)
    }

    fn eval_ident_path(&mut self, node: &Node) -> Option<Value> {
        let mut children = Children::new(node.children.iter().copied(), self.nodes);
        let (left_id, _) = children.find_node(Nodes::Value)?;
        let left = self.eager_eval(left_id, false)?;
        let _ = children.find_node(Nodes::Op)?;
        let (right_id, right) = children.find_node(Nodes::Identifier)?;
        let key = &self.input[right.span];

        let mut map = self.expect_some(left_id, left.into_struct(), Error::ValueNotStruct)?;
        self.expect_some(right_id, map.remove(key), Error::FieldNotFound)
    }

    fn eval_self_ident_path(&mut self, op_id: NodeId, value_id: NodeId, value: &Node) -> Option<Value> {
        let text = &self.input[value.span];
        let current = self.nodes.ancestors(op_id)
            .find(|ancestor| {
                self.nodes.get(ancestor).is(Nodes::Struct)
            });
        let current = self.expect_some(op_id, current, Error::ContextNotFound)?;
        let current = self.eval(current)?;
        let mut map = self.expect_some(op_id, current.into_struct(), Error::ValueNotStruct)?;
        self.expect_some(value_id, map.remove(text), Error::FieldNotFound)
    }

    fn eval_unary(&mut self, node: &Node) -> Option<Value> {
        let mut children = Children::new(node.children.iter().copied(), self.nodes);
        let (op_id, op) = children.find_node(Nodes::Op)?;
        let text_op = &self.input[op.span];

        let (value_id, value) = children.find_node(Nodes::Value)?;

        if text_op == "." { return self.eval_self_ident_path(op_id, value_id, value); }

        let value = self.eager_eval(value_id, false)?;
        match (text_op, value) {
            ("-", Value::Number(i))  => Some(Value::Number(-i)),
            ("!", Value::Boolean(b))  => Some(Value::Boolean(!b)),
            _ => unreachable!()
        }
    }

    fn eval_binary(&mut self, node: &Node) -> Option<Value> {
        let mut children = Children::new(node.children.iter().copied(), self.nodes);
        let (left, _) = children.find_node(Nodes::Value)?;
        let left = self.eager_eval(left, false)?;
        let (_, op) = children.find_node(Nodes::Op)?;
        let (right, _) = children.find_node(Nodes::Value)?;
        let right = self.eager_eval(right, false)?;
        let text_op = &self.input[op.span];
        match (left, text_op, right) {
            (Value::Number(l), "-", Value::Number(r))  => Some(Value::Number(l - r)),
            (Value::Number(l), "+", Value::Number(r))  => Some(Value::Number(l + r)),
            (Value::Number(l), "*", Value::Number(r))  => Some(Value::Number(l * r)),
            (Value::Number(l), "/", Value::Number(r))  => Some(Value::Number(l / r)),
            (Value::Boolean(l), "==", Value::Boolean(r))  => Some(Value::Boolean(l == r)),
            _ => unreachable!()
        }
    }

    fn eval(&mut self, id: NodeId) -> Option<Value> {
        let node = self.nodes.get(id);

        if node.is(Nodes::Root) {
            return node.children.iter()
                .filter_map(|child| self.eval(*child))
                .next();
        }
        if !node.is(Nodes::Value) { return None; }

        let mut children = Children::new(node.children.iter().copied(), self.nodes);
        let text = &self.input[node.span];

        if node.is(Nodes::Identifier) { return self.eval_identifier(id, node); }
        if node.is(Nodes::IdentPath) { return self.eval_ident_path(node); }
        if node.is(Nodes::Number) { return Some(Value::Number(text.parse().unwrap())); }
        if node.is(Nodes::Boolean) { return Some(Value::Boolean(text == "true")); }
        if node.is(Nodes::Unary) { return self.eval_unary(node); }
        if node.is(Nodes::Binary) { return self.eval_binary(node); }

        if node.is(Nodes::Array) {
            let mut values = vec![];
            while let Some((value, _)) = children.find_node(Nodes::Value) {
                let value = self.eval(value)?;
                values.push(value);
            }
            return Some(Value::Array(values));
        }

        if node.is(Nodes::Struct) {
            let mut map = BTreeMap::default();
            while let Some((_, key)) = children.find_node(Nodes::Key) {
                let key = self.input[key.span].to_string();
                let (value, _) = children.find_node(Nodes::Value)?;
                let value = Value::Lazy { id: value };
                map.insert(key, value);
            }
            return Some(Value::Struct(map));
        }

        if node.is(Nodes::Markdown) {
            let mut s = String::new();
            while let Some((_, value)) = children.find_node(Nodes::Md_Value) {
                self.eval_md(&mut s, value)?;
            }
            return Some(Value::String(s));
        }

        if node.is(Nodes::String) {
            let mut s = String::new();
            while let Some((_, value)) = children.find_node(Nodes::StrValue) {
                if value.is(Nodes::Interpolated) {
                    let mut children = Children::new(value.children.iter().copied(), self.nodes);
                    let (value_id, _) = children.find_node(Nodes::Value)?;
                    let value = self.eager_eval(value_id, true)?;
                    s += &value.to_string();
                } else {
                    s += &self.input[value.span];
                }
            }
            return Some(Value::String(s));
        }
        if node.is(Nodes::Parens) {
            let (value, _) = children.find_node(Nodes::Value)?;
            return self.eval(value);
        }

        None
    }
}

pub fn eval(id: NodeId, nodes: &Arena, input: &str) -> EvalResult {
    let mut eval = Eval::new(nodes, input);
    let value = eval.eval(id)
        .and_then(|val| {
            eval.into_eager(val, true)
        });
    EvalResult { value, errors: eval.errors }
}

#[cfg(test)]
mod tests {
    use super::*;
    use neu_parser::State;
    use neu_syntax::{neu::parser, MainLexer};

    #[test]
    fn eval_tests() {
        test_runner::test_snapshots("neu", "eval", |input| {
            let lexer = MainLexer::new(input);

            let res = State::parse(lexer, parser());
            let result = eval(res.root, &res.nodes, input);

            result.display(input).to_string()
        }).unwrap();
    }

}
