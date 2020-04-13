pub mod core;

mod token {
    use derive_more::Display;
    #[derive(Debug, PartialEq, Clone, Copy, Display)]
    pub enum Token {
        #[display(fmt = "error")]
        Error,

        #[display(fmt = "` `, `\n`, `\t`")]
        Trivia, // Whitespace

        #[display(fmt = "number")]
        Number,

        #[display(fmt = "`true`")]
        True,

        #[display(fmt = "`false`")]
        False,

        #[display(fmt = "`-`")]
        OpMinus,

        #[display(fmt = "`!`")]
        OpBang,
    }
}

pub mod lexer {
    use crate::core::{Input, TextRange};
    use crate::Token;

    pub fn lex(input: &mut Input) -> Option<(Token, TextRange)> {
        let i = input.as_ref();
        let peeked = i.chars().next()?;
        if peeked.is_whitespace() {
            let rest = i.chars().take_while(|c| c.is_whitespace()).count();

            return Some((Token::Trivia, input.chomp(rest)));
        }
        if peeked.is_ascii_digit() {
            let rest = i.chars().take_while(|c| c.is_ascii_digit()).count();

            return Some((Token::Number, input.chomp(rest)));
        }

        if peeked == '-' {
            return Some((Token::OpMinus, input.chomp(1)));
        }

        if peeked == '!' {
            return Some((Token::OpBang, input.chomp(1)));
        }

        if i.starts_with("true") {
            return Some((Token::True, input.chomp(4)));
        }

        if i.starts_with("false") {
            return Some((Token::False, input.chomp(5)));
        }

        Some((Token::Error, input.chomp(1)))
    }
}

pub mod nodes {
    use crate::core::Name;
    use crate::nodes;

    nodes! {
        Value,
        Number,
        Boolean,
        Unary,
        Op
    }
}

pub use crate::lexer::*;
pub use crate::nodes::*;
pub use crate::token::*;

use crate::core::*;
pub use crate::{Nodes, Token};

pub fn parser() -> impl Parser {
    node(|builder| {
        builder.name(Nodes::Root);
        let trivia = trivia();
        let ctx = Context::new(&trivia);
        builder.parse_ctx(&ctx, value());
        builder.parse(token(None));
    })
}

fn value() -> impl Parser {
    const VALUE_TOKENS: &[Token] = &[
        Token::Number, Token::True, Token::False,
        Token::OpMinus, Token::OpBang
    ];

    node(|builder| {
        builder.name(Nodes::Virtual);
        builder.name(Nodes::Value);
        match builder.peek_token() {
            Some(Token::Number) => builder.parse(number()),
            Some(Token::True)
            | Some(Token::False) => builder.parse(boolean()),
            Some(Token::OpMinus)
            | Some(Token::OpBang) => builder.parse(unary()),
            _ => builder.parse( expected( VALUE_TOKENS))
        };
    })
}

fn unary() -> impl Parser {
    node(|builder| {
        builder.name(Nodes::Unary);
        builder.parse( tokens(vec![Token::OpMinus, Token::OpBang])
                          .map(|n| n.with_name(Nodes::Op))
        );
        builder.parse(value());
    })
}

fn boolean() -> impl Parser {
    tokens(vec![Token::True, Token::False]).map(|n| n.with_name(Nodes::Boolean))
}

fn number() -> impl Parser {
    token(Token::Number).map(|n| n.with_name(Nodes::Number))
}

fn trivia() -> impl Parser {
    node_trivia(|builder| {
        builder.name(Nodes::Trivia);
        let mut empty = true;
        while let Some(Token::Trivia) = builder.peek_token() {
            builder.next_token();
            empty = false;
        }

        if empty {
            builder.name(Nodes::Virtual);
        }
    })
}

