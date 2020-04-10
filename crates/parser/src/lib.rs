pub mod core;

mod token {
    #[derive(Debug, PartialEq, Clone, Copy)]
    pub enum Token {
        Error,
        Atom,
        Trivia, // Whitespace
        Open,
        Close,
    }
}

pub mod lexer {
    use crate::core::{Input, TextRange};
    use crate::Token;

    pub fn lex(i: &mut Input) -> Option<(Token, TextRange)> {
        Some(match i.as_ref().chars().next()? {
            c if c.is_whitespace() => {
                let rest = i.as_ref().chars().take_while(|c| c.is_whitespace()).count();
                (Token::Trivia, i.chomp(rest))
            }
            c if c.is_alphanumeric() => {
                let rest = i
                    .as_ref()
                    .chars()
                    .take_while(|c| c.is_alphanumeric())
                    .count();
                (Token::Atom, i.chomp(rest))
            }
            '(' => (Token::Open, i.chomp(1)),
            ')' => (Token::Close, i.chomp(1)),
            _ => (Token::Error, i.chomp(1)),
        })
    }
}

pub mod nodes {
    use crate::core::Name;

    pub struct Nodes;
    #[allow(non_upper_case_globals)]
    impl Nodes {
        pub const Virtual: Name = Name::new("Virtual");
        pub const Root: Name = Name::new("Root");
        pub const Trivia: Name = Name::new("Trivia");
        pub const Token: Name = Name::new("Token");
        pub const Error: Name = Name::new("Error");
        pub const SExp: Name = Name::new("SExp");
        pub const List: Name = Name::new("List");
        pub const Nil: Name = Name::new("Nil");
        pub const Atom: Name = Name::new("Atom");
    }
}

pub use crate::lexer::*;
pub use crate::nodes::*;
pub use crate::token::*;

pub fn parser() -> impl Parser {
    node(|builder, state, ctx| {
        builder.name(Nodes::Root);
        builder.parse(state, ctx, parse_sexp());
        builder.parse(state, ctx, token(None));
    })
}

use crate::core::*;
pub use crate::{Nodes, Token};

fn trivia() -> impl Parser {
    node(|builder, state, _| {
        builder.name(Nodes::Trivia);
        let mut empty = true;
        while let Some(Token::Trivia) = state.lexer_mut().peek().as_kind() {
            state.lexer_mut().next();
            empty = false;
        }

        if empty {
            builder.name(Nodes::Virtual);
        }
    })
}

fn parse_sexp() -> impl Parser {
    const SEXP_TOKENS: &[Token] = &[Token::Atom, Token::Open];

    node(|builder, state, _| {
        builder.name(Nodes::Virtual);
        builder.name(Nodes::SExp);
        let trivia = trivia();
        let ctx = Context::new(&trivia);
        let ctx = &ctx;
        let peeked = state.lexer_mut().peek();
        match peeked.as_kind() {
            None => {
                builder.parse(
                    state,
                    ctx,
                    node(|builder, state, _| {
                        state.lexer_mut().next();
                        builder.error(Error::UnexpectedEOF {
                            expected: SEXP_TOKENS.to_vec(),
                        });
                    }),
                );
            }
            Some(Token::Atom) => builder.parse(
                state,
                ctx,
                token(Token::Atom).map(|node| node.with_name(Nodes::Atom)),
            ),
            Some(Token::Open) => builder.parse(state, ctx, parse_list()),
            Some(_) => {
                builder.parse(
                    state,
                    ctx,
                    node(|builder, state, _| {
                        let token = state.lexer_mut().next();
                        builder.error(Error::UnexpectedToken {
                             expected: SEXP_TOKENS.to_vec(),
                             found: token.as_kind().unwrap(),
                        });
                    }),
                );
            }
        };
    })
}

fn parse_list() -> impl Parser {
    node(|builder, state, ctx| {
        builder.parse(state, ctx, token(Token::Open));
        let peeked = state.lexer_mut().peek();
        match peeked.as_kind() {
            None => {
                builder.error(Error::UnexpectedEOF {
                    expected: vec![Token::Close, Token::Atom],
                });
            }
            Some(Token::Close) => {
                builder.name(Nodes::Nil);
            }
            _ => loop {
                builder.name(Nodes::List);
                builder.parse(state, ctx, parse_sexp());
                let peeked = state.lexer_mut().peek();
                match peeked.as_kind() {
                    None => {
                        break;
                    }
                    Some(Token::Close) => break,
                    _ => continue,
                }
            },
        }
        builder.parse(state, ctx, token(Token::Close));
    })
}
