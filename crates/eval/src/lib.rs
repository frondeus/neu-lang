mod children;
mod value;

use neu_parser::core::{NodeId, Arena};
use neu_parser::Nodes;
use value::Value;
use children::Children;
use std::collections::BTreeMap;

#[derive(Clone, Copy, Default, Debug)]
struct Context {
    top: Option<NodeId>
}

impl Context {
    pub fn top(&self) -> Option<NodeId> {
        self.top
    }

    pub fn current(self, id: NodeId) -> Self {
        let mut top = self.top;

        if top.is_none() {
            top = Some(id);
        }

        Self {
            top
        }
    }
}

struct Eval<'a> {
    pub nodes: &'a Arena,
    pub input: &'a str
}

impl<'a> Eval<'a> {
    fn to_eager(&self, value: Value, ctx: Context) -> Option<Value> {
        match value {
            Value::Lazy { id, parent } => {
                let ctx = ctx.current(parent);
                let v = self.eval(id, ctx)?;
                self.to_eager(v, ctx)
            },
            Value::Struct(s) => {
                let s = s.into_iter()
                    .map(|(k, v)| {
                        let v = self.to_eager(v, ctx);
                        v.map(|v| (k, v))
                    }).collect::<Option<BTreeMap<String, Value>>>()?;
                Some(Value::Struct(s))
            },
            Value::Array(a) => {
                let a = a.into_iter()
                    .map(|v| {
                        self.to_eager(v, ctx)
                    }).collect::<Option<Vec<Value>>>()?;
                Some(Value::Array(a))
            },
            v => Some(v)
        }
    }

    fn eager_eval(&self, id: NodeId, ctx: Context) -> Option<Value> {
        let v = self.eval(id, ctx)?;
        self.to_eager(v, ctx)
    }

    fn eval(&self, id: NodeId, ctx: Context) -> Option<Value> {
        let node = self.nodes.get(id);

        if node.is(Nodes::Root) {
            return node.children.iter()
                .filter_map(|child| self.eval(*child, ctx))
                .next();
        }
        if !node.is(Nodes::Value) { return None; }

        let mut children = Children::new(node.children.iter().copied(), self.nodes);
        let text = &self.input[node.span];

        if node.is(Nodes::Identifier) {
            let top = ctx.top().and_then(|top| {
                self.eval(top, ctx)
            }).and_then(|v| v.as_struct())
            .and_then(|mut map| {
                map.remove(text)
            });
            return top;
        }

        if node.is(Nodes::IdentPath) {
            let (left, _) = children.find_node(Nodes::Value)?;
            let left = self.eager_eval(left, ctx)?;
            let _ = children.find_node(Nodes::Op)?;
            let (_, right) = children.find_node(Nodes::Identifier)?;
            let key = &self.input[right.span];

            let mut map = left.as_struct()?;
            return map.remove(key);
        }


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
            let value = self.eager_eval(value, ctx)?;
            let text_op = &self.input[op.span];
            return match (text_op, value) {
                ("-", Value::Number(i))  => Some(Value::Number(-i)),
                ("!", Value::Boolean(b))  => Some(Value::Boolean(!b)),
                _ => None
            };
        }

        if node.is(Nodes::Binary) {
            let (left, _) = children.find_node(Nodes::Value)?;
            let left = self.eager_eval(left, ctx)?;
            let (_, op) = children.find_node(Nodes::Op)?;
            let (right, _) = children.find_node(Nodes::Value)?;
            let right = self.eager_eval(right, ctx)?;
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

        if node.is(Nodes::Array) {
            let mut values = vec![];
            while let Some((value, _)) = children.find_node(Nodes::Value) {
                let value = self.eval(value, ctx)?;
                values.push(value);
            }
            return Some(Value::Array(values));
        }

        if node.is(Nodes::Struct) {
            let mut map = BTreeMap::default();
            while let Some((_, key)) = children.find_node(Nodes::Key) {
                let key = self.input[key.span].to_string();
                let (value, _) = children.find_node(Nodes::Value)?;
                let value = Value::Lazy { id: value, parent: id };
                map.insert(key, value);
            }
            return Some(Value::Struct(map));
        }

        if node.is(Nodes::Parens) {
            let (value, _) = children.find_node(Nodes::Value)?;
            return self.eval(value, ctx);
        }

        None
    }
}

pub fn eval(id: NodeId, nodes: &Arena, input: &str) -> Option<Value> {
    let eval = Eval { nodes, input };
    eval.eval(id, Context::default())
        .and_then(|val| {
            let ctx = Context::default();
            let ctx = ctx.current(id);
            eval.to_eager(val, ctx)
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use neu_parser::core::{Lexer, State};
    use neu_parser::parser;

    #[test]
    fn eval_tests() {
        test_runner::test_snapshots("eval", |input| {
            let lexer = Lexer::new(input);

            let res = State::parse(lexer, parser());
            let result = eval(res.root, &res.nodes, input);

            match result {
                None => "None".to_string(),
                Some(r) => format!("`{:#}`", r)
            }
        }).unwrap();
    }

}
