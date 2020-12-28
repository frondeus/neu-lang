use crate::parsers::common::separated;
use crate::parsers::markdown::inner_md_string;
use crate::{
    lexers::{neu::Token, string::Token as StrToken},
    Nodes,
};
use microtree_parser::*;
use microtree_parser::parsers::*;

pub fn parser<'s>() -> impl Parser<'s, Token> {
    node(|builder| {
        let leading_trivia = leading_trivia();
        let trailing_trivia = trailing_trivia();
        let ctx = Context {
            leading_trivia: Some(&leading_trivia),
            trailing_trivia: Some(&trailing_trivia),
        };
        builder.node()
        .name(Nodes::Root)
        .set_ctx(&ctx)
        .parse(value())
        .parse(token(None))
        .finish()
    })
}

pub(crate) fn value<'s>() -> impl Parser<'s, Token> + Clone {
    Pratt::new(
        left_value(),
        |token| match token {
            Some(Token::OpDot) => Some((Assoc::Left, 100)),

            Some(Token::OpStar) => Some((Assoc::Left, 20)),
            Some(Token::OpSlash) => Some((Assoc::Left, 20)),

            Some(Token::OpMinus) => Some((Assoc::Left, 10)),
            Some(Token::OpPlus) => Some((Assoc::Left, 10)),

            Some(Token::OpDEqual) => Some((Assoc::Left, 1)),
            _ => None,
        },
        |builder, op_token| {
            let mut builder = builder.name(Nodes::Value);
            match op_token {
                Some(Token::OpDot) => {
                    builder.name(Nodes::IdentPath)
                }
                _ => {
                    builder.name(Nodes::Binary)
                }
            }
            .parse(named(token(op_token), Nodes::BinaryOp))
        },
    )
    .parser()
}

fn left_value<'s>() -> impl Parser<'s, Token> + Clone {
    const VALUE_TOKENS: &[Token] = &[
        Token::Number,
        Token::True,
        Token::False,
        Token::OpMinus,
        Token::OpBang,
        Token::DoubleQuote,
        Token::OpenP,
        Token::OpenC,
        Token::OpenB,
        Token::Identifier,
    ];

    node(|builder| {
        let mut builder = builder.name(Nodes::Value);

        match builder.peek_token() {
            Some(Token::Number) => builder.parse(number()),
            Some(Token::True) | Some(Token::False) => builder.parse(boolean()),
            Some(Token::OpMinus) | Some(Token::OpBang) | Some(Token::OpDot) => {
                builder.parse(unary())
            }
            Some(Token::MdQuote) => builder.parse(md_string()),
            Some(Token::DoubleQuote) => builder.parse(string()),
            Some(Token::OpenC) => builder.parse(strukt()),
            Some(Token::OpenB) => builder.parse(array()),
            Some(Token::Identifier) => builder.parse(identifier()),
            Some(Token::OpenP) => builder.parse(node(|builder| {
                builder.node()
                .parse(token(Token::OpenP))
                .parse(value())
                .parse(token(Token::CloseP))
                .finish()
            })),
            _ => builder.parse(tokens(VALUE_TOKENS.to_vec())),
        }
    })
}

fn array<'s>() -> impl Parser<'s, Token> {
    node(|builder| {
        builder.node()
        .name(Nodes::Array)
        .parse(token(Token::OpenB))
        .parse(separated(value(), Token::Comma, Token::CloseB, true))
        .parse(token(Token::CloseB))
        .finish()
    })
}

fn identifier<'s>() -> impl Parser<'s, Token> + Clone {
    named(token(Token::Identifier), Nodes::Identifier)
}

pub(crate) fn strukt_key<'s>() -> impl Parser<'s, Token> {
    named(identifier(), Nodes::Key)
}

fn strukt<'s>() -> impl Parser<'s, Token> {
    node(|builder| {
        builder.node()
        .name(Nodes::Strukt)
        .parse(token(Token::OpenC))
        .parse(separated(
            node(|builder| {
                builder.node()
                .name(Nodes::StruktPair)
                .parse(strukt_key())
                .parse(token(Token::OpAssign))
                .parse(value())
                .finish()
            }),
            Token::Comma,
            Token::CloseC,
            true,
        ))
        .parse(token(Token::CloseC))
        .finish()
    })
}

fn md_string<'s>() -> impl Parser<'s, Token> {
    node(|builder| {
        let trivia = leading_trivia();
        let ctx = Context {
            leading_trivia: Some(&trivia),
            .. Default::default()
        };
        let prev_ctx = builder.get_ctx();
        builder.node()
        .name(Nodes::Markdown)
        .set_ctx(&ctx)
        .parse(token(Token::MdQuote))
        .parse_mode(inner_md_string(), None)
        .set_ctx(prev_ctx)
        .parse(token(Token::DoubleQuote))
        .finish()
    })
}

fn string<'s>() -> impl Parser<'s, Token> {
    node(|builder| {
        let trivia = leading_trivia();
        let ctx = Context {
            leading_trivia: Some(&trivia),
            .. Default::default()
        };
        let prev_ctx = builder.get_ctx();
        builder.node()
        .name(Nodes::String)
        .set_ctx(&ctx)
        .parse(token(Token::DoubleQuote))
        .parse_mode(inner_string(), None)
        .set_ctx(prev_ctx)
        .parse(token(Token::DoubleQuote))
        .finish()
    })
}

fn inner_string<'s>() -> impl Parser<'s, StrToken> {
    node(|builder| {
        let mut builder = builder.node()
            .name(Nodes::InnerString);
        loop {
            match builder.peek_token() {
                Some(StrToken::CloseI) | Some(StrToken::Text) => {
                    builder = builder.parse(any_token());
                    continue;
                }
                Some(StrToken::OpenI) => {
                    builder = builder.parse(node(|builder| {
                        let leading_trivia = leading_trivia();
                        let trailing_trivia = trailing_trivia();
                        let ctx = Context {
                            leading_trivia: Some(&leading_trivia),
                            trailing_trivia: Some(&trailing_trivia),
                        };
                        builder.node()
                        .name(Nodes::Interpolated)
                        .parse(token(StrToken::OpenI))
                        .parse_mode(value(), &ctx)
                        .parse(token(StrToken::CloseI))
                        .finish()
                    }));
                }
                None | Some(StrToken::Close) => break builder.finish(),
            }
        }
    })
}

fn unary<'s>() -> impl Parser<'s, Token> {
    node(|builder| {
        builder.node()
        .name(Nodes::Unary)
        .parse(named(
            tokens(vec![Token::OpMinus, Token::OpBang, Token::OpDot]),
            Nodes::UnaryOp,
        ))
        .parse(value())
        .finish()
    })
}

fn boolean<'s>() -> impl Parser<'s, Token> {
    named(tokens(vec![Token::True, Token::False]), Nodes::Boolean)
}

fn number<'s>() -> impl Parser<'s, Token> {
    named(token(Token::Number), Nodes::Number)
}

pub(crate) fn trailing_trivia<'s>() -> impl Trivia<'s, Token> {
    |lexer: &mut Lexer<'s, Token>| {
        while let Some(Token::Whitespace) | Some(Token::InlineComment) | Some(Token::BlockComment) = lexer.peek_token() {
            lexer.next();
        }
    }
}

pub(crate) fn leading_trivia<'s>() -> impl Trivia<'s, Token> {
    |lexer: &mut Lexer<'s, Token>| {
        while let Some(Token::LineEnd) = lexer.peek_token() {
            lexer.next();
        }
        while let Some(Token::Whitespace)
            | Some(Token::InlineComment)
            | Some(Token::BlockComment) = lexer.peek_token() {
                lexer.next();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::parser;
    use crate::lexers::neu::{Lexer, Token};
    use microtree_parser::{ParseResult, State, Spanned};

    #[test]
    fn lexer_tests() {
        test_runner::test_snapshots("neu", "lexer", |input| {
            let lexer = Lexer::new(input);

            let res: Vec<_> = lexer
                .map(|t: Spanned<Token>| format!("{:?}", t))
                .collect();
            format!("{:#?}", res)
        })
        .unwrap();
    }

    #[test]
    fn parser_tests() {
        test_runner::test_snapshots("neu", "parser", |input| {
            let lexer = Lexer::new(input);

            let res: ParseResult = State::parse(lexer, parser());

            if let Some(root) = res.root {
                format!("{:?}", root)
            } else {
                "No tree".to_string()
            }
        })
        .unwrap();
    }
}
