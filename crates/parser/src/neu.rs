use crate::core::*;
use crate::{MainLexer, Nodes, Token, StringLexer, StrToken};

pub fn parser() -> impl Parser<MainLexer> {
    node(|builder| {
        builder.name(Nodes::Root);
        let trivia = trivia();
        let ctx = Context::new(&trivia);
        builder.parse_ctx(&ctx, value());
        builder.parse(token(None));
    })
}

fn value() -> impl Parser<MainLexer> {
    let next = |state: &mut State<_>, ctx: &Context<_>| left_value().parse(state, ctx);
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

fn left_value() -> impl Parser<MainLexer> {
    const VALUE_TOKENS: &[Token] = &[
        Token::Number, Token::True, Token::False,
        Token::OpMinus, Token::OpBang, Token::OpenS, Token::OpenP,
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
            Some(Token::OpenS) => builder.parse(string()),
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

fn array() -> impl Parser<MainLexer> {
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

fn identifier() -> impl Parser<MainLexer> {
    token(Token::Identifier).map(|n| n.with_name(Nodes::Identifier))
}

fn strukt_key() -> impl Parser<MainLexer> {
    identifier().map(|n| n.with_name(Nodes::Key))
}

fn strukt() -> impl Parser<MainLexer> {
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

fn string() -> impl Parser<MainLexer> {
    node(|builder: &mut NodeBuilder<'_, MainLexer>| {
        builder.name(Nodes::String);
        let ctx = Context::default();
        builder.parse_ctx(&ctx, token(Token::OpenS));
        let ctx2 = Context::default();
        builder.parse_mode(&ctx2, inner_string());
        builder.parse_ctx(&ctx, token(Token::OpenS));
    })
}

fn inner_string() -> impl Parser<StringLexer> {
    node(|builder| {
        builder.name(Nodes::Virtual);
        builder.name(Nodes::StrValue);
        loop {
            match builder.peek_token() {
                Some(StrToken::CloseI) | Some(StrToken::Text) => {
                    builder.parse(any_token());
                    continue
                },
                Some(StrToken::OpenI) => {
                    builder.parse(node(|builder| {
                        builder.name(Nodes::Interpolated);
                        builder.parse(token(StrToken::OpenI));
                        let trivia = trivia();
                        let ctx = Context::new(&trivia);
                        builder.parse_mode(&ctx, value());
                        builder.parse(token(StrToken::CloseI));
                    }));
                },
                None | Some(StrToken::Close) => break,
            }
        }
    })
}

fn unary() -> impl Parser<MainLexer> {
    node(|builder| {
        builder.name(Nodes::Unary);
        builder.parse( tokens(vec![Token::OpMinus, Token::OpBang, Token::OpDot])
            .map(|n| n.with_name(Nodes::Op))
        );
        builder.parse(value());
    })
}

fn boolean() -> impl Parser<MainLexer> {
    tokens(vec![Token::True, Token::False]).map(|n| n.with_name(Nodes::Boolean))
}

fn number() -> impl Parser<MainLexer> {
    token(Token::Number).map(|n| n.with_name(Nodes::Number))
}

fn trivia() -> impl Parser<MainLexer> {
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
    use crate::core::{State, Lexer};
    use super::{parser, MainLexer};

    #[test]
    fn lexer_tests() {
        test_runner::test_snapshots("neu", "lexer", |input| {
            let lexer = MainLexer::new(input);

            let res: Vec<_> = lexer.into_iter().map(|t| t.display(input, true).to_string()).collect();
            format!("{:#?}", res)
        }).unwrap();
    }

    #[test]
    fn parser_tests() {
        test_runner::test_snapshots("neu", "parser", |input| {
            let lexer = MainLexer::new(input);

            let res = State::parse(lexer, parser());

            format!("{}", res.display(input))
        }).unwrap();
    }
}
