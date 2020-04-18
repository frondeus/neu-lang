pub mod core;

mod token {
    use derive_more::Display;
    #[derive(Debug, PartialEq, Clone, Copy, Display)]
    pub enum Token {
        #[display(fmt = "error")]
        Error,

        #[display(fmt = "` `, `\t`, `\n`")]
        Whitespace,

        #[display(fmt = "comment")]
        Comment,

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

        #[display(fmt = "`=`")]
        OpAssign,

        #[display(fmt = "identifier")]
        Identifier,

        #[display(fmt = "string")]
        String,

        #[display(fmt = "`(`")]
        OpenP,

        #[display(fmt = "`)`")]
        CloseP,

        #[display(fmt = "`{{`")]
        OpenC,

        #[display(fmt = "`}}`")]
        CloseC,

        #[display(fmt = "`[`")]
        OpenB,

        #[display(fmt = "`]`")]
        CloseB,

        #[display(fmt = "`,`")]
        Comma,

        #[display(fmt = "`.`")]
        OpDot
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

            return Some((Token::Whitespace, input.chomp(rest)));
        }
        if peeked.is_ascii_digit() {
            let rest = i.chars().take_while(|c| c.is_ascii_digit()).count();

            return Some((Token::Number, input.chomp(rest)));
        }

        if i.starts_with("/*") {
            let mut peeked = peeked;
            let mut i = &i[2..];
            let mut rest = 2;
            while !i.starts_with("*/") {
                i = &i[peeked.len_utf8()..];
                rest += 1;
                peeked = match i.chars().next() {
                    Some(p) => p,
                    None => return Some((Token::Error, input.chomp(2)))
                }
            }
            rest += 2;
            return Some((Token::Comment, input.chomp(rest)));
        }

        if i.starts_with("//") {
            let rest = i.chars().take_while(|c| *c != '\n').count();
            return Some((Token::Comment, input.chomp(rest)));
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
        if peeked == '=' { return Some((Token::OpAssign, input.chomp(1))); }
        if peeked == '.' { return Some((Token::OpDot, input.chomp(1))); }

        if peeked == ',' { return Some((Token::Comma, input.chomp(1))); }

        if peeked == '(' { return Some((Token::OpenP, input.chomp(1))); }
        if peeked == ')' { return Some((Token::CloseP, input.chomp(1))); }
        if peeked == '{' { return Some((Token::OpenC, input.chomp(1))); }
        if peeked == '}' { return Some((Token::CloseC, input.chomp(1))); }
        if peeked == '[' { return Some((Token::OpenB, input.chomp(1))); }
        if peeked == ']' { return Some((Token::CloseB, input.chomp(1))); }

        if peeked == '"' {
            let rest = i.chars().skip(1).take_while(|c| *c != '"').count();

            return Some((Token::String, input.chomp(rest + 2)));
        }

        if peeked.is_ascii_alphabetic() {
            let rest = i.chars()
                .take_while(|c| c.is_ascii_alphanumeric() || *c == '_').count();
            return Some((Token::Identifier, input.chomp(rest)));
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
        Op,

        Struct,
        Identifier,
        Key,

        Array,
        IdentPath
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
        Some(Token::OpDot) => Some((Assoc::Left, 100)),

        Some(Token::OpStar) => Some((Assoc::Left, 20)),
        Some(Token::OpSlash) => Some((Assoc::Left, 20)),

        Some(Token::OpMinus) => Some((Assoc::Left, 10)),
        Some(Token::OpPlus) => Some((Assoc::Left, 10)),

        Some(Token::OpDEqual) => Some((Assoc::Left, 1)),
        _ => None
    }, |builder, op_token| {
        match op_token {
            Some(Token::OpDot) => {
                builder.name(Nodes::IdentPath);
            },
            _ => {
                builder.name(Nodes::Binary);
            }
        }
        builder.name(Nodes::Value);
        builder.parse(token(op_token).map(|n| n.with_name(Nodes::Op)));
    }).parser()
}

fn left_value() -> impl Parser {
    const VALUE_TOKENS: &[Token] = &[
        Token::Number, Token::True, Token::False,
        Token::OpMinus, Token::OpBang, Token::String, Token::OpenP,
        Token::OpenC, Token::OpenB, Token::Identifier
    ];

    node(|builder| {
        builder.name(Nodes::Virtual);
        builder.name(Nodes::Value);
        match builder.peek_token() {
            Some(Token::Number) => builder.parse(number()),
            Some(Token::True) | Some(Token::False) => builder.parse(boolean()),
            Some(Token::OpMinus)
            | Some(Token::OpBang)
            | Some(Token::OpDot)
                => builder.parse(unary()),
            Some(Token::String) => builder.parse(string()),
            Some(Token::OpenC) => builder.parse(strukt()),
            Some(Token::OpenB) => builder.parse(array()),
            Some(Token::Identifier) => builder.parse(identifier()),
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

fn array() -> impl Parser {
    node(|builder| {
        builder.name(Nodes::Array);
        builder.parse(token(Token::OpenB));
        match builder.peek_token() {
            Some(Token::CloseB) => (),
            _ => 'outer: loop {
                builder.parse(value());
                'inner: loop {
                    match builder.peek_token() {
                        None | Some(Token::CloseB) => { break 'outer; },
                        Some(Token::Comma) => {
                            builder.parse(token(Token::Comma)); //recover(","));
                            if let Some(Token::CloseB) = builder.peek_token() { // Trailing comma
                                break 'outer;
                            }
                            break 'inner;
                        },
                        _ => {
                            builder.parse(tokens(vec![Token::Comma, Token::CloseB]));
                        }
                    }
                }
            }
        }
        builder.parse(token(Token::CloseB));
    })
}

fn identifier() -> impl Parser {
    token(Token::Identifier).map(|n| n.with_name(Nodes::Identifier))
}

fn strukt_key() -> impl Parser {
    identifier().map(|n| n.with_name(Nodes::Key))
}

fn strukt() -> impl Parser {
    node(|builder| {
        builder.name(Nodes::Struct);
        builder.parse(token(Token::OpenC));
        match builder.peek_token() {
            Some(Token::CloseC) => (),
            _ => 'outer: loop {
                builder.parse(strukt_key());
                builder.parse(token(Token::OpAssign));
                builder.parse(value());
                'inner: loop {
                    match builder.peek_token() {
                        None | Some(Token::CloseC) => {
                            break 'outer
                        },
                        Some(Token::Comma) => {
                            builder.parse(token(Token::Comma)); //recover(","));
                            if let Some(Token::CloseC) = builder.peek_token() { // Trailing comma
                                break 'outer;
                            }
                            break 'inner;
                        }
                        _ => {
                            builder.parse(tokens(vec![Token::Comma, Token::CloseC]));
                        }
                    }
                }
            }
        }
        builder.parse(token(Token::CloseC));
    })
}

fn string() -> impl Parser {
    token(Token::String).map(|n| n.with_name(Nodes::String))
}

fn unary() -> impl Parser {
    node(|builder| {
        builder.name(Nodes::Unary);
        builder.parse( tokens(vec![Token::OpMinus, Token::OpBang, Token::OpDot])
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
        while let Some(Token::Whitespace)
        | Some(Token::Comment) = builder.peek_token() {
            builder.next_token();
            empty = false;
        }

        if empty {
            builder.name(Nodes::Virtual);
        }
    })
}

#[cfg(test)]
mod tests {
    use crate::core::{Lexer, State};
    use crate::parser;

    #[test]
    fn lexer_tests() {
        test_runner::test_snapshots("lexer", |input| {
            let lexer = Lexer::new(input);

            let res: Vec<_> = lexer.map(|t| t.display(input, true).to_string()).collect();
            format!("{:#?}", res)
        }).unwrap();
    }

    #[test]
    fn parser_tests() {
        test_runner::test_snapshots("parser", |input| {
            let lexer = Lexer::new(input);

            let res = State::parse(lexer, parser());

            format!("{}", res.display(input))
        }).unwrap();
    }
}