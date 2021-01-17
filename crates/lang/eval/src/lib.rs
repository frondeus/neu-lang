mod error;
mod markdown;
mod result;
mod value;

pub mod db;

use error::Error;
use neu_syntax::{ast::{ArticleBody, ArticleBodyItem, ArticleRef, Binary, BinaryOp, IdentPath, Identifier, InnerStringPart, Markdown, OpDot, Strukt, SubArticle, Unary, UnaryOp, Value as AstValue}, reexport::{Ast, Red, SmolStr}};
use neu_diagnostics::{Diagnostic, ToReport, Diagnostics};
use std::collections::BTreeMap;
pub use value::Value;

pub struct Eval {
    pub errors: Diagnostics,
    pub red: Red
}

impl Eval {
    pub fn new(red: Red) -> Self {
        Self {
            red,
            errors: Default::default()
        }
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn into_eager(&mut self, value: Value, recursive: bool) -> Option<Value> {
        match value {
            Value::Lazy { red } => {
                let v = self.eval_red(red)?;
                if !recursive {
                    return Some(v);
                }
                self.into_eager(v, recursive)
            }
            Value::Struct(s) => {
                let s = s
                    .into_iter()
                    .map(|(k, v)| {
                        let v = self.into_eager(v, recursive);
                        v.map(|v| (k, v))
                    })
                    .collect::<Option<BTreeMap<SmolStr, Value>>>()?;
                Some(Value::Struct(s))
            }
            Value::Array(a) => {
                let a = a
                    .into_iter()
                    .map(|v| self.into_eager(v, recursive))
                    .collect::<Option<Vec<Value>>>()?;
                Some(Value::Array(a))
            }
            v => Some(v),
        }
    }

    fn eager_eval(&mut self, red: Red, recursive: bool) -> Option<Value> {
        let v = self.eval_red(red)?;
        self.into_eager(v, recursive)
    }

    fn eval(&mut self) -> Option<Value> {
        self.eval_red(self.red.clone())
    }

    fn str_non_trivia(red: Red) -> SmolStr {
        let green = red.green();
        let green = green.as_token().unwrap();
        green.value.clone()
    }

    pub fn eval_red(&mut self, red: Red) -> Option<Value> {
        if let Some(value) = AstValue::new(red.clone()) {
            return match value {
                AstValue::Number(num) => {
                    let number = Self::str_non_trivia(num.red()).parse().unwrap();
                    Some(Value::Number(number))
                },
                AstValue::Boolean(boolean) => {
                    let s = Self::str_non_trivia(boolean.red());
                    Some(Value::Boolean(s == "true"))
                }
                AstValue::IdentPath(ident_path) => {
                    self.eval_ident_path(ident_path)
                }
                AstValue::Unary(unary) => {
                    self.eval_unary(unary)
                }
                AstValue::Binary(binary) => {
                    self.eval_binary(binary)
                },
                AstValue::Array(array) => {
                    let values = array.values()
                                      .map(|val|
                                           self.eval_red(val.red())
                                      )
                        .collect::<Option<Vec<_>>>()?;
                    Some(Value::Array(values))
                },
                AstValue::Strukt(strukt) => {
                    let map = strukt
                        .pairs()
                        .map(|pair| {
                            let key = pair.key()?;
                            let value = pair.value()?;
                            let key = Self::str_non_trivia(key.red());
                            let value = Value::Lazy { red: value.red() };
                            Some((key, value))
                        })
                        .collect::<Option<BTreeMap<_, _>>>()?;

                    Some(Value::Struct(map))
                },
                AstValue::Identifier(ident) => {
                    self.eval_identifier(ident)
                },
                AstValue::String(string) => {
                    let parts = string.inner_string()?
                    .parts()
                    .map(|part| match part {
                        InnerStringPart::Text(text) => {
                            Some(Self::str_non_trivia(text.red()))
                        },
                        InnerStringPart::Interpolated(interpolated) => {
                            let value = interpolated.value()?;
                            let value = self.eager_eval(value.red(), true)?;
                            Some(value.to_string().into())
                        }
                    })
                    .collect::<Option<Vec<_>>>()?;
                    let s = parts.join("");
                    Some(Value::String(s.into()))
                },
                AstValue::MdString(string) => {
                    let mut s = String::default();
                    if let Some(markdown) = string.markdown() {
                        //let references =
                        for value in markdown.red().children() {
                            self.eval_md(&mut s, value)?;
                        }
                    }
                    Some(Value::String(s.into()))
                }
                s => {
                    eprintln!("TODO: {:?}", s);
                    None
                }
            };
        }
        if let Some(body) = ArticleBody::new(red.clone()) {
            let mut s = String::default();
            let red = body.red();
            for item in red.pre_order() {
                if let Some(sub) = SubArticle::new(item.clone()) {
                }
                else if let Some(re) = ArticleRef::new(item.clone()) {
                }
                else if let Some(md) = Markdown::new(item) {
                    for value in md.red().children() {
                        self.eval_md(&mut s, value)?;
                    }
                }
            }
            return Some(Value::String(s.into()));
        }
        dbg!(&red);
        /*
        if node.is(Nodes::Root) {
            return node
                .children
                .iter()
                .filter_map(|child| self.eval(*child))
                .next();
        }

        if node.is(Nodes::Parens) {
            let (value, _) = children.find_node(Nodes::Value)?;
            return self.eval(value);
        }

        */
        None
    }

    fn eval_unary(&mut self, unary: Unary) -> Option<Value> {
        let op = unary.unary_op()?;

        let value = unary.value()?;

        if let UnaryOp::OpDot(op) = op {
            return self.eval_self_ident_path(op, value);
        }

        let value = self.eager_eval(value.red(), false)?;
        match (op, value) {
            (UnaryOp::OpMinus(_), Value::Number(i)) => Some(Value::Number(-i)),
            (UnaryOp::OpBang(_), Value::Boolean(b)) => Some(Value::Boolean(!b)),
            _ => todo!(),
        }
    }

    fn eval_self_ident_path(
        &mut self,
        op: OpDot,
        value: AstValue,
    ) -> Option<Value> {
        let current =
            op.red()
              .ancestors()
            .filter_map(|ancestor| Strukt::new(ancestor)).next();
        let current = self.expect_some(op.red(), current, Error::ContextNotFound)?;
        let current = self.eval_red(current.red())?;
        let mut map = self.expect_some(op.red(), current.into_struct(), Error::ValueNotStruct)?;
        let key = Self::str_non_trivia(value.red());
        self.expect_some(value.red(), map.remove(&key), Error::FieldNotFound)
    }

    fn eval_binary(&mut self, binary: Binary) -> Option<Value> {
        let left = binary.left()?;
        let left = self.eager_eval(left.red(), false)?;

        let op = binary.binary_op()?;

        let right = binary.right()?;
        let right = self.eager_eval(right.red(), false)?;

        match (left, op, right) {
            (Value::Number(l), BinaryOp::OpMinus(_), Value::Number(r)) => Some(Value::Number(l - r)),
            (Value::Number(l), BinaryOp::OpPlus(_), Value::Number(r)) => Some(Value::Number(l + r)),
            (Value::Number(l), BinaryOp::OpStar(_), Value::Number(r)) => Some(Value::Number(l * r)),
            (Value::Number(l), BinaryOp::OpSlash(_), Value::Number(r)) => Some(Value::Number(l / r)),
            (Value::Boolean(l), BinaryOp::OpDEqual(_), Value::Boolean(r)) => Some(Value::Boolean(l == r)),
            _ => unreachable!(),
        }
    }

    fn expect_some<V>(&mut self, red: Red, v: Option<V>, error: Error) -> Option<V> {
        match v {
            Some(v) => Some(v),
            None => {
                let err: Diagnostic = error.to_report();
                self.errors.add(red.range(), err);
                None
            }
        }
    }

    fn eval_identifier(&mut self, ident: Identifier) -> Option<Value> {
        let key = Self::str_non_trivia(ident.red());
        let top = ident.red().ancestors()
                             .filter_map(|red| Strukt::new(red))
            .last();
        let top = self.expect_some(ident.red(), top, Error::ContextNotFound)?;
        let top = self.eval_red(top.red())?;
        let mut map = self.expect_some(ident.red(), top.into_struct(), Error::ValueNotStruct)?;
        self.expect_some(ident.red(), map.remove(&key), Error::FieldNotFound)
    }

    fn eval_ident_path(&mut self, ident_path: IdentPath) -> Option<Value> {
        let left = ident_path.left()?;
        let left_val = self.eager_eval(left.red(), false)?;

        let right = ident_path.right()?.as_identifier()?;
        let key = Self::str_non_trivia(right.red());

        let mut map = self.expect_some(left.red(), left_val.into_struct(), Error::ValueNotStruct)?;
        self.expect_some(right.red(), map.remove(&key), Error::FieldNotFound)
    }


}

#[cfg(test)]
mod tests {
    use crate::db::Evaluator;
    use neu_syntax::db::{FileKind, Parser};
    use std::sync::Arc;

    #[salsa::database(crate::db::EvaluatorDatabase, neu_syntax::db::ParserDatabase)]
    #[derive(Default)]
    struct TestDb {
        storage: salsa::Storage<Self>,
    }

    impl salsa::Database for TestDb {}

    #[test]
    fn eval_tests() {
        test_runner::test_snapshots("neu", "eval", |input| {
            let mut db = TestDb::default();
            let path = db.file_id(("test".into(), FileKind::Neu));
            db.set_all_mds(Default::default());
            db.set_all_neu(Arc::new(Some(path.clone()).into_iter().collect()));
            db.set_input(path.clone(), Arc::new(input.into()));
            let result = db.eval_file(path.clone());

            format!("{}", result)
        })
        .unwrap();
    }
}
