use std::{fmt::Display, marker::PhantomData};
use microtree::Green;

use crate::{Builder, Name, Parser, State, TokenKind};

pub fn node<'s, Tok: TokenKind<'s>>(
    f: impl FnMut(Builder<'s, '_, Tok>) ->
        (Option<Green>, State<'s, Tok>) + Clone
)
    -> impl Parser<'s, Tok> + Clone {
    f
}

pub fn node_once<'s, Tok: TokenKind<'s>>(
    f: impl FnMut(Builder<'s, '_, Tok>) ->
        (Option<Green>, State<'s, Tok>)
)
    -> impl Parser<'s, Tok> {
    f
}

pub fn direct<'s, Tok: TokenKind<'s>>(
    green: Option<Green>
) -> impl Parser<'s, Tok> {
    Direct { _phantom: PhantomData, green }
}

pub struct Direct<Tok> {
    _phantom: PhantomData<Tok>,
    green: Option<Green>
}
impl<'source, Tok> Parser<'source, Tok> for Direct<Tok> where Tok: TokenKind<'source> {
    fn parse<'ctx>(&mut self, state: State<'source, Tok>,
                   _context: &crate::Context<'source, 'ctx, Tok>) -> (Option<Green>, State<'source, Tok>) {
        (self.green.clone(), state)
    }
}


pub fn named<'s, Tok: TokenKind<'s>>(parser: impl Parser<'s, Tok> + Clone, name: Name)
                                     -> impl Parser<'s, Tok> + Clone {
    node(move |builder| builder.name(name).parse(parser.clone()))
}

pub fn any_token<'s, Tok: TokenKind<'s>>() -> impl Parser<'s, Tok> {
    |builder: Builder<'s, '_, Tok>| builder.name(Name::new("token")).token()
}

pub fn error<'s, Tok: TokenKind<'s>>(desc: impl ToString + Clone) -> impl Parser<'s, Tok> {
    move |builder: Builder<'s, '_, Tok>| builder.error(desc.clone())
}

pub fn tokens<'s, Tok: TokenKind<'s>>(expected: Vec<Tok>) -> impl Parser<'s, Tok> + Clone {
    let expect_eof = expected.is_empty();
    move |mut builder: Builder<'s, '_, Tok>| match (builder.peek_token(), expect_eof) {
        (Some(tok), true) => builder.error(format!("Expected EOF, found {}", tok)),
        (None, false) => builder.error(format!("{} but found EOF`", Expected::new(&expected))),
        (Some(tok), false) if !expected.contains(&tok) => {
            builder.error(format!("{} but found {}", Expected::new(&expected), tok))
        }
        _ => builder.name(Name::new("token")).token(),
    }
}

pub fn token<'s, Tok: TokenKind<'s>>(expected: impl Into<Option<Tok>>)
                                     -> impl Parser<'s, Tok> + Clone {
    let expected = expected.into();
    move |mut builder: Builder<'s, '_, Tok>| match (builder.peek_token(), expected) {
        (Some(tok), None) => builder.error(format!("Expected EOF, found {}", tok)),
        (None, Some(expected)) => builder.error(format!("Expected {} but found EOF`", expected)),
        (Some(tok), Some(expected)) if tok != expected => {
            builder.error(format!("Expected {} but found {}", expected, tok))
        }
        _ => builder.name(Name::new("token")).token(),
    }
}


struct Expected<'a, 's, Tok: TokenKind<'s>>{
    expected: &'a [Tok],
    _phantom: PhantomData<&'s ()>
}

impl <'a, 's, Tok> Expected<'a, 's, Tok>
where Tok: TokenKind<'s>
{
    fn new(expected: &'a [Tok]) -> Self {
        Self {
            expected,
            _phantom: PhantomData
        }
    }
}

impl<'a, 's, Tok> Display for Expected<'a, 's, Tok>
where Tok: TokenKind<'s>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Expected ")?;
        let last = self.expected.len() - 1;
        if last > 0 {
            write!(f, "one of ")?;
        }
        let iter = self.expected.iter();
        for (i, token) in iter.enumerate() {
            if i == 0 {
                write!(f, "{}", token)?;
            } else {
                write!(f, ", {}", token)?;
            }
        }
        Ok(())
    }
}
