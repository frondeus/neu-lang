use crate::{Context, Name, Node, NodeBuilder, OptionExt, ParseError, Parser, State, TokenKind};
use crate::{CoreNodes as Nodes, CoreNodes};
use std::cell::RefCell;
use std::marker::PhantomData;

pub enum Assoc {
    Right,
    Left,
}
pub struct Pratt<N, BP, F, Tok> {
    next: N,
    bp: BP,
    f: F,
    rbp: i32,
    _phantom: PhantomData<Tok>,
}

impl<N, BP, F, Tok> Clone for Pratt<N, BP, F, Tok>
where
    Tok: TokenKind,
    N: Clone + Parser<Tok>,
    BP: Clone + Fn(Option<Tok>) -> Option<(Assoc, i32)>,
    F: Clone + Fn(&mut NodeBuilder<Tok>, Option<Tok>),
{
    fn clone(&self) -> Self {
        Self {
            next: self.next.clone(),
            bp: self.bp.clone(),
            f: self.f.clone(),
            rbp: self.rbp,
            _phantom: PhantomData,
        }
    }
}

impl<N, BP, F, Tok> Pratt<N, BP, F, Tok>
where
    Tok: TokenKind,
    N: Clone + Parser<Tok>,
    BP: Clone + Fn(Option<Tok>) -> Option<(Assoc, i32)>,
    F: Clone + Fn(&mut NodeBuilder<Tok>, Option<Tok>),
{
    pub fn new(next: N, bp: BP, f: F) -> Self {
        Self {
            next,
            bp,
            f,
            rbp: 0,
            _phantom: PhantomData,
        }
    }

    pub fn rbp(&self, rbp: i32) -> Self {
        let mut new = self.clone();
        new.rbp = rbp - 1;
        new
    }

    pub fn parser(&self) -> impl Parser<Tok> + Clone {
        let opt = self.clone();
        move |state: &mut State<Tok>, ctx: &Context<Tok>| {
            let mut builder = NodeBuilder::new(state, ctx);
            builder.parse(opt.next.clone()); // Left

            loop {
                let op_token = builder.peek_token();
                let (op_assoc, op_bp) = match (opt.bp)(op_token.as_ref().copied()) {
                    Some(op) if op.1 > opt.rbp => op,
                    _ => {
                        builder.name(CoreNodes::Virtual);
                        return builder.build().0;
                    }
                };

                (opt.f)(&mut builder, op_token);
                let new_op_bp = match op_assoc {
                    Assoc::Left => op_bp + 1,
                    Assoc::Right => op_bp - 1,
                };
                builder.parse(opt.rbp(new_op_bp).parser());

                let (left, state, ctx) = builder.build();
                builder = NodeBuilder::new(state, ctx);
                builder.add(left);
            }
        }

        /*
        move |state: &mut State<Tok>, ctx: &Context<Tok>| {
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
         */
    }
}

pub fn node_mut<Tok: TokenKind>(mut f: impl FnMut(&mut NodeBuilder<Tok>)) -> impl Parser<Tok> {
    NodeMut {
        f: RefCell::new(move |state: &mut State<Tok>, ctx: &Context<Tok>| {
            let mut builder = NodeBuilder::new(state, ctx);
            f(&mut builder);
            builder.build().0
        }),
    }
}

pub fn node<Tok: TokenKind>(f: impl Fn(&mut NodeBuilder<Tok>) + Clone) -> impl Parser<Tok> + Clone {
    move |state: &mut State<Tok>, ctx: &Context<Tok>| {
        let mut builder = NodeBuilder::new(state, ctx);
        f(&mut builder);
        builder.build().0
    }
}

pub fn expected<Tok: TokenKind + Send>(expected: &'static [Tok]) -> impl Parser<Tok> {
    node(move |builder| {
        let found = builder.next_token();
        builder.error(ParseError::Expected {
            found,
            expected: expected.to_vec(),
        });
    })
}

pub fn tokens<Tok: TokenKind + Send + 'static>(expected: Vec<Tok>) -> impl Parser<Tok> {
    node(move |builder: &mut NodeBuilder<Tok>| {
        builder.name(Nodes::Token);
        let token = builder.next_token();
        match (token.as_kind(), expected.is_empty()) {
            (None, false) => {
                builder.error(ParseError::Expected {
                    expected: expected.clone(),
                    found: None,
                });
            }
            (Some(_), true) => {
                builder.error(ParseError::ExpectedEOF {
                    found: token.unwrap(),
                });
            }
            (Some(found), false) if !expected.contains(&found) => {
                builder.error(ParseError::Expected {
                    found: Some(token.unwrap()),
                    expected: expected.clone(),
                });
            }
            _ => (),
        };
    })
}

pub fn any_token<Tok: TokenKind>() -> impl Parser<Tok> {
    node(move |builder: &mut NodeBuilder<Tok>| {
        builder.name(Nodes::Token);
        builder.next_token();
    })
}

pub fn token<Tok: TokenKind + Send + 'static>(
    expected: impl Into<Option<Tok>>,
) -> impl Parser<Tok> {
    let expected = expected.into();
    let expected = expected.into_iter().collect::<Vec<_>>();
    tokens(expected)
}

pub fn named<Tok: TokenKind>(parser: impl Parser<Tok>, name: Name) -> impl Parser<Tok> {
    parser.map(move |node| node.with_name(name))
}

pub trait ParserExt<Tok: TokenKind>: Parser<Tok> {
    fn map<F>(self, f: F) -> Map<Self, F>
    where
        F: Fn(Node) -> Node,
        Self: Sized;

    fn boxed(self) -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(self)
    }
}

impl<P, Tok> ParserExt<Tok> for P
where
    Tok: TokenKind,
    P: Parser<Tok>,
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

impl<P, Fun, Tok> Parser<Tok> for Map<P, Fun>
where
    Tok: TokenKind,
    P: Parser<Tok>,
    Fun: Fn(Node) -> Node,
{
    fn parse(&self, state: &mut State<Tok>, ctx: &Context<Tok>) -> Node {
        let o = self.parser.parse(state, ctx);
        (self.f)(o)
    }
}

struct NodeMut<F> {
    f: RefCell<F>,
}

impl<Tok: TokenKind, F> Parser<Tok> for NodeMut<F>
where
    F: FnMut(&mut State<Tok>, &Context<Tok>) -> Node,
{
    fn parse(&self, state: &mut State<Tok>, ctx: &Context<Tok>) -> Node {
        let mut f = self.f.borrow_mut();
        let f = &mut *f;
        f(state, ctx)
    }
}
