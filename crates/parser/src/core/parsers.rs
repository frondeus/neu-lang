use crate::core::{Context, Error, Node, NodeBuilder, OptionExt, Lexer, Parser, State};
use crate::Nodes;
use std::marker::PhantomData;

pub enum Assoc {
    Right, Left
}
pub struct Pratt<N, BP, F, Lex> {
    next: N,
    bp: BP,
    f: F,
    rbp: i32,
    _phantom: PhantomData<Lex>
}

impl<N, BP, F, Lex> Clone for Pratt<N, BP, F, Lex>
where
    Lex: Lexer,
    N: Clone + Parser<Lex>,
    BP: Clone + Fn(Option<Lex::Token>) -> Option<(Assoc, i32)>,
    F: Clone + Fn(&mut NodeBuilder<Lex>, Option<Lex::Token>)
{
    fn clone(&self) -> Self {
        Self {
            next: self.next.clone(),
            bp: self.bp.clone(),
            f: self.f.clone(),
            rbp: self.rbp,
            _phantom: PhantomData
        }
    }
}

impl<N, BP, F, Lex> Pratt<N, BP, F, Lex>
where
    Lex: Lexer,
    N: Clone + Parser<Lex>,
    BP: Clone + Fn(Option<Lex::Token>) -> Option<(Assoc, i32)>,
    F: Clone + Fn(&mut NodeBuilder<Lex>, Option<Lex::Token>)
{
    pub fn new(next: N, bp: BP, f: F) -> Self {
        Self { next, bp, f, rbp: 0, _phantom: PhantomData }
    }

    pub fn rbp(&self, rbp: i32) -> Self {
        let mut new = self.clone();
        new.rbp = rbp - 1;
        new
    }

    pub fn parser(&self) -> impl Parser<Lex> {
        let opt = self.clone();

        move |state: &mut State<Lex>, ctx: &Context<Lex>| {
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

pub fn node<Lex: Lexer>(f: impl Fn(&mut NodeBuilder<Lex>)) -> impl Parser<Lex> {
    move |state: &mut State<Lex>, ctx: &Context<Lex>| {
        let mut builder = NodeBuilder::new(state, ctx);
        f(&mut builder);
        builder.build()
    }
}

pub fn expected<Lex: Lexer>(expected: &'static [Lex::Token]) -> impl Parser<Lex> {
    node(move |builder| {
        let found = builder.next_token();
        builder.error(Error::Expected {
            found,
            expected: expected.to_vec()
        });
    })
}

pub fn tokens<Lex: Lexer>(expected: Vec<Lex::Token>) -> impl Parser<Lex> {
    node(move |builder: &mut NodeBuilder<Lex>| {
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

pub fn token<Lex: Lexer>(expected: impl Into<Option<Lex::Token>>) -> impl Parser<Lex> {
    let expected = expected.into();
    let expected = expected.into_iter().collect::<Vec<_>>();
    tokens(expected)
}


pub trait ParserExt<Lex: Lexer>: Parser<Lex> {
    fn map<F>(self, f: F) -> Map<Self, F>
    where
        F: Fn(Node) -> Node,
        Self: Sized;

    fn boxed(self) -> Box<Self> where Self: Sized {
        Box::new(self)
    }
}

impl<P, Lex> ParserExt<Lex> for P
where
    Lex: Lexer,
    P: Parser<Lex>,
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

impl<P, Fun, Lex> Parser<Lex> for Map<P, Fun>
where
    Lex: Lexer,
    P: Parser<Lex>,
    Fun: Fn(Node) -> Node,
{
    fn parse(&self, state: &mut State<Lex>, ctx: &Context<Lex>) -> Node {
        let o = self.parser.parse(state, ctx);
        (self.f)(o)
    }
}
