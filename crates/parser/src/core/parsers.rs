use crate::core::{Context, Error, Node, NodeBuilder, OptionExt, Parser, State};
use crate::{Nodes, Token};

pub fn node(f: impl Fn(&mut NodeBuilder, &mut State, &Context)) -> impl Parser {
    move |state: &mut State, ctx: &Context| {
        let mut builder = Node::builder(state.lexer());
        f(&mut builder, state, ctx);
        builder.build(state.lexer())
    }
}

pub fn error(error: Error) -> impl Parser {
    move |state: &mut State, ctx: &Context| {
        let mut builder = Node::builder(state.lexer());
        builder.error(error.clone());
        builder.build(state.lexer())
    }
}

pub fn token(expected: impl Into<Option<Token>>) -> impl Parser {
    let expected = expected.into();
    let expected = expected.into_iter().collect::<Vec<_>>();
    node(move |builder: &mut NodeBuilder, state, _| {
        builder.name(Nodes::Token);
        let token = state.lexer_mut().next();
        match (token.as_kind(), expected.is_empty()) {
            (None, false) => {
                builder.error(Error::UnexpectedEOF {
                    expected: expected.clone(),
                });
            }
            (Some(found), true) => {
                builder.error(Error::ExpectedEOF { found });
            }
            (Some(found), false) if !expected.contains(&found) => {
                builder.error(Error::UnexpectedToken {
                    found,
                    expected: expected.clone(),
                });
            }
            _ => (),
        };
    })
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
