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

        #[display(fmt = "`+`")]
        OpPlus,

        #[display(fmt = "`*`")]
        OpStar,

        #[display(fmt = "`/`")]
        OpSlash,

        #[display(fmt = "`==`")]
        OpDEqual,

        #[display(fmt = "string")]
        String,

        #[display(fmt = "`(`")]
        OpenP,

        #[display(fmt = "`)`")]
        CloseP
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

        if i.starts_with("==") {
            return Some((Token::OpDEqual, input.chomp(2)));
        }

        if i.starts_with("true") {
            return Some((Token::True, input.chomp(4)));
        }

        if i.starts_with("false") {
            return Some((Token::False, input.chomp(5)));
        }

        if peeked == '-' { return Some((Token::OpMinus, input.chomp(1))); }
        if peeked == '!' { return Some((Token::OpBang, input.chomp(1))); }
        if peeked == '+' { return Some((Token::OpPlus, input.chomp(1))); }
        if peeked == '*' { return Some((Token::OpStar, input.chomp(1))); }
        if peeked == '/' { return Some((Token::OpSlash, input.chomp(1))); }
        if peeked == '(' { return Some((Token::OpenP, input.chomp(1))); }
        if peeked == ')' { return Some((Token::CloseP, input.chomp(1))); }

        if peeked == '"' {
            let rest = i.chars().skip(1).take_while(|c| *c != '"').count();

            return Some((Token::String, input.chomp(rest + 2)));
        }

        Some((Token::Error, input.chomp(1)))
    }
}

pub mod nodes {
    use crate::core::Name;
    use crate::nodes;

    nodes! {
        Value,
        Parens,
        Number,
        Boolean,
        String,

        Unary,
        Binary,
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
    let next = |state: &mut State, ctx: &Context| left_value().parse(state, ctx);
    Pratt::new(next, |token| match token {
        Some(Token::OpStar) => Some(20),
        Some(Token::OpSlash) => Some(20),

        Some(Token::OpMinus) => Some(10),
        Some(Token::OpPlus) => Some(10),

        Some(Token::OpDEqual) => Some(1),
        _ => None
    }, |builder, op_token| {
        builder.name(Nodes::Binary);
        builder.name(Nodes::Value);
        builder.parse(token(op_token).map(|n| n.with_name(Nodes::Op)));
    }).parser()
}

fn left_value() -> impl Parser {
    const VALUE_TOKENS: &[Token] = &[
        Token::Number, Token::True, Token::False,
        Token::OpMinus, Token::OpBang, Token::String, Token::OpenP
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
            Some(Token::String) => builder.parse(string()),
            Some(Token::OpenP) => {
                builder.parse(node(|builder| {
                    builder.name(Nodes::Parens);
                    builder.parse(token(Token::OpenP));
                    builder.parse(value());
                    builder.parse(token(Token::CloseP));
                }))
            }
            _ => builder.parse( expected( VALUE_TOKENS))
        };
    })
}

fn string() -> impl Parser {
    token(Token::String).map(|n| n.with_name(Nodes::String))
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
    node(|builder| {
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

