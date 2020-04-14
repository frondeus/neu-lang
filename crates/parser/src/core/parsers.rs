use crate::core::{Context, Error, Node, NodeBuilder, OptionExt, Parser, State};
use crate::{Nodes, Token};

pub enum Assoc {
    Right, Left
}
pub struct Pratt<N, BP, F> {
    next: N,
    bp: BP,
    f: F,
    rbp: i32,
}

impl<N, BP, F> Clone for Pratt<N, BP, F>
where
    N: Clone + Parser,
    BP: Clone + Fn(Option<Token>) -> Option<(Assoc, i32)>,
    F: Clone + Fn(&mut NodeBuilder, Option<Token>)
{
    fn clone(&self) -> Self {
        Self {
            next: self.next.clone(),
            bp: self.bp.clone(),
            f: self.f.clone(),
            rbp: self.rbp
        }
    }
}

impl<N, BP, F> Pratt<N, BP, F>
where
    N: Clone + Parser,
    BP: Clone + Fn(Option<Token>) -> Option<(Assoc, i32)>,
    F: Clone + Fn(&mut NodeBuilder, Option<Token>)
{
    pub fn new(next: N, bp: BP, f: F) -> Self {
        Self { next, bp, f, rbp: 0 }
    }

    pub fn rbp(&self, rbp: i32) -> Self {
        let mut new = self.clone();
        new.rbp = rbp - 1;
        new
    }

    pub fn parser(&self) -> impl Parser {
        use crate::core::peekable::PeekableIterator;
        let opt = self.clone();

        move |state: &mut State, ctx: &Context| {
            let mut left = opt.next.parse(state, ctx);
            loop {
                let op_token = state.lexer_mut().peek().as_kind();
                let (op_assoc, op_bp) = match (opt.bp)(op_token.as_ref().copied()) {
                    Some(op) if op.1 > opt.rbp => op,
                    _ => return left,
                };

                let mut builder = NodeBuilder::new(state, ctx);
                builder.add(left);
                (opt.f)(&mut builder, op_token);
                let new_op_bp = match op_assoc {
                    Assoc::Left => op_bp + 1,
                    Assoc::Right => op_bp - 1,
                };
                builder.parse(opt.rbp(new_op_bp).parser());
                left = builder.build();
            }
        }
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

    fn boxed(self) -> Box<Self> where Self: Sized {
        Box::new(self)
    }
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
