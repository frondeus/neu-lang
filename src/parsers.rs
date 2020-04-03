use crate::node_builder::NodeBuilder;
use crate::lexer::{Lexer, OptionExt};
use crate::parser::{Parser, Context};
use crate::token::TokenKind;
use crate::node::{Node, Nodes};

pub fn node<Lex, K>(f: impl Fn(&mut NodeBuilder, &mut Context<Lex>)) -> impl Parser<Lex, K>
    where
        Lex: Lexer<K>,
        K: TokenKind,
{
    move |context: &mut Context<Lex>| {
        let mut node = Node::builder(context);
        f(&mut node, context);
        node.build(context)
    }
}

pub fn v_node<Lex, K>(f: impl Fn(&mut NodeBuilder, &mut Context<Lex>)) -> impl Parser<Lex, K>
    where
        Lex: Lexer<K>,
        K: TokenKind,
{
    move |context: &mut Context<Lex>| {
        let mut node = Node::builder(context);
        node.name(Nodes::Virtual);
        f(&mut node, context);
        node.build(context)
    }
}

pub fn token<Lex, K>(expected: impl Into<Option<K>>) -> impl Parser<Lex, K>
    where
        Lex: Lexer<K>,
        K: TokenKind,
{
    let expected = expected.into();
    node(move |node, context: &mut Context<Lex>| {
        node.name(Nodes::Token);
        let token = context.lexer_mut().next();
        match (token.as_kind(), expected.as_ref()) {
            (None, Some(expected)) => {
                node.name(Nodes::Error);
                context.error(format!("Expected {:?} but found EOF", expected));
            }
            (Some(kind), None) => {
                node.name(Nodes::Error);
                context.error(format!("Expected EOF but found {:?}", kind));
            }
            (Some(kind), Some(expected)) if kind != expected => {
                node.name(Nodes::Error);
                context.error(format!("Expected {:?} but found {:?}", expected, kind));
            }
            _ => (),
        };
    })
}

pub fn trivia<Lex, K>() -> impl Parser<Lex, K>
    where
        Lex: Lexer<K>,
        K: TokenKind,
{
    node(|node, context: &mut Context<Lex>| {
        node.name(Nodes::Trivia);
        context.lexer_mut().next();
    })
}

pub trait ParserExt<Lex, K> {
    fn map<F>(self, f: F) -> Map<Self, F>
        where
            F: Fn(Node) -> Node,
            Self: Sized;
}

impl<P, Lex, K> ParserExt<Lex, K> for P
    where
        P: Parser<Lex, K>,
        Lex: Lexer<K>,
        K: TokenKind,
{
    fn map<F>(self, f: F) -> Map<Self, F>
        where
            F: Fn(Node) -> Node,
            Self: Sized,
    {
        Map {
            f,
            parser: self,
        }
    }
}

pub struct Map<P, F> {
    parser: P,
    f: F,
}

impl<K, P, Fun, Lex> Parser<Lex, K> for Map<P, Fun>
    where
        P: Parser<Lex, K>,
        K: TokenKind,
        Fun: Fn(Node) -> Node,
        Lex: Lexer<K>,
{
    fn parse(&self, context: &mut Context<Lex>) -> Node {
        let o = self.parser.parse(context);
        (self.f)(o)
    }
}
