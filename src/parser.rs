use crate::lexer::Lexer;
use crate::node::Node;
use crate::token::TokenKind;
use std::fmt::Display;

pub type Error = String;

pub struct Context<Lex> {
    lexer: Lex,
    errors: Vec<Error>
}

impl<Lex> Context<Lex> {
    fn new(lexer: Lex) -> Self {
        Self {
            lexer ,
            errors: Default::default()
        }
    }

    pub fn parse<K>(lexer: Lex, parser: impl Parser<Lex, K>) -> ParseResult
    where Lex: Lexer<K>, K: TokenKind
    {
        let mut context = Self::new(lexer);
        let node = parser.parse(&mut context);

        ParseResult { node, errors: context.errors }
    }

    pub fn lexer_mut(&mut self) -> &mut Lex {
        &mut self.lexer
    }

    pub fn lexer(&self) -> &Lex {
        &self.lexer
    }

    pub fn error(&mut self, d: impl Display) {
        self.errors.push(d.to_string());
    }
}


#[derive(Debug)]
pub struct ParseResult {
    pub node: Node,
    pub errors: Vec<Error>
}

pub trait Parser<Lex, K>
where
    K: TokenKind,
    Lex: Lexer<K>,
{
    fn parse(&self, context: &mut Context<Lex>) -> Node;
}

impl<K, Fun, Lex> Parser<Lex, K> for Fun
where
    K: TokenKind,
    Fun: Fn(&mut Context<Lex>) -> Node,
    Lex: Lexer<K>,
{
    fn parse(&self, context: &mut Context<Lex>) -> Node {
        self(context)
    }
}
