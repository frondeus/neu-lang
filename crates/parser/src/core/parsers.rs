use crate::core::{Context, Error, Node, NodeBuilder, OptionExt, Parser, State};
use crate::{Nodes, Token};

pub fn node_trivia(f: impl Fn(&mut NodeBuilder)) -> impl Parser {
    move |state: &mut State, ctx: &Context| {
        let mut builder = NodeBuilder::new(state, ctx);
        f(&mut builder);
        builder.build()
    }
}

pub fn node(f: impl Fn(&mut NodeBuilder)) -> impl Parser {
    move |state: &mut State, ctx: &Context| {
        let mut builder = NodeBuilder::new(state, ctx);
        f(&mut builder);
        builder.build()
    }
}

pub fn expected(expected: &'static [Token]) -> impl Parser {
    node(move |builder| {
        let found = builder.next_token();
        builder.error(Error::Expected {
            found,
            expected: expected.to_vec()
        });
    })
}

pub fn tokens(expected: Vec<Token>) -> impl Parser {
    node(move |builder: &mut NodeBuilder| {
        builder.name(Nodes::Token);
        let token = builder.next_token();
        match (token.as_kind(), expected.is_empty()) {
            (None, false) => {
                builder.error(Error::Expected {
                    expected: expected.clone(),
                    found: None
                });
            }
            (Some(_), true) => {
                builder.error(Error::ExpectedEOF { found: token.unwrap() });
            }
            (Some(found), false) if !expected.contains(&found) => {
                builder.error(Error::Expected {
                    found: Some(token.unwrap()),
                    expected: expected.clone(),
                });
            }
            _ => (),
        };
    })
}
pub fn token(expected: impl Into<Option<Token>>) -> impl Parser {
    let expected = expected.into();
    let expected = expected.into_iter().collect::<Vec<_>>();
    tokens(expected)
}

pub trait ParserExt: Parser {
    fn map<F>(self, f: F) -> Map<Self, F>
    where
        F: Fn(Node) -> Node,
        Self: Sized;
}

impl<P> ParserExt for P
where
    P: Parser,
{
    fn map<F>(self, f: F) -> Map<Self, F>
    where
        F: Fn(Node) -> Node,
        Self: Sized,
    {
        Map { f, parser: self }
    }
}

pub struct Map<P, F> {
    parser: P,
    f: F,
}

impl<P, Fun> Parser for Map<P, Fun>
where
    P: Parser,
    Fun: Fn(Node) -> Node,
{
    fn parse(&self, state: &mut State, ctx: &Context) -> Node {
        let o = self.parser.parse(state, ctx);
        (self.f)(o)
    }
}
