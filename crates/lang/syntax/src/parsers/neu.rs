use crate::parsers::markdown::inner_md_string;
use crate::{
    lexers::{neu::Token, string::Token as StrToken},
    Nodes,
};
use microtree_parser::parsers::*;
use microtree_parser::*;

pub fn parser<S: Sink>() -> impl Parser<Token, S> + Clone {
    parse(|s| {
        let leading = leading_trivia();
        let trailing = trailing_trivia();
        s.with_ctx(
            Context {
                leading: Some(&leading),
                trailing: Some(&trailing),
            },
            value(),
        )
        .expect(None)
    })
}

pub(crate) fn value< S: Sink >() -> impl Parser< Token, S> + Clone {
    pratt(
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
        |s, op_token| {
            s.alias(Nodes::Value)
                .start(match op_token {
                    Some(Token::OpDot) => Nodes::IdentPath,
                    _ => Nodes::Binary,
                })
                .alias(Nodes::BinaryOp)
                .token()
        },
    )
}

fn left_value<S: Sink >() -> impl Parser<Token, S> + Clone {
    use Token::*;
    parse(|s| {
        s.peek()
         .at(OpenP).parse(parse(|s| s.token().parse(value()).expect(CloseP) ))
         .parse_else(parse(|s| s.alias(Nodes::Value)))
         .at(Number).parse(number())
         .at(True).at(False).parse(boolean())
         .at(OpMinus).at(OpBang).at(OpDot).parse(unary())
         .at(MdQuote).parse( md_string())
         .at(DoubleQuote).parse(string())
         .at(OpenC).parse(strukt())
         .at(OpenB).parse(array())
         .at(Identifier).parse(identifier())
         .expect()
    })
}

fn array<S: Sink >() -> impl Parser<Token, S> {
    parse(|s| {
        s.start(Nodes::Array)
            .token()
            .parse(separated(value(), Token::Comma, Token::CloseB, true))
            .expect(Token::CloseB)
            .end()
    })
}

fn identifier<S: Sink >() -> impl Parser<Token, S> + Clone {
    parse(|s| s.alias(Nodes::Identifier).expect(Token::Identifier))
}

pub(crate) fn strukt_key<S: Sink >() -> impl Parser<Token, S> {
    parse(|s| s.alias(Nodes::Key).parse(identifier()))
}

fn strukt<S: Sink >() -> impl Parser<Token, S> {
    parse(|s| {
        s.start(Nodes::Strukt)
            .token()
            .parse(separated(
                parse(|s| {
                    s.start(Nodes::StruktPair)
                        .parse(strukt_key())
                        .expect(Token::OpAssign)
                        .parse(value())
                        .end()
                }),
                Token::Comma,
                Token::CloseC,
                true,
            ))
            .expect(Token::CloseC)
            .end()
    })
}

fn md_string<S: Sink >() -> impl Parser<Token, S> {
    parse(|s| {
        let trivia = leading_trivia();
        s.start(Nodes::MdString)
            .with_ctx(
                Context {
                    leading: Some(&trivia),
                    ..Default::default()
                },
                parse(|s| s.token().with_mode(inner_md_string())),
            )
            .expect(Token::DoubleQuote)
            .end()
    })
}

fn string<S: Sink >() -> impl Parser<Token, S> {
    parse(|s| {
        let trivia = leading_trivia();
        s.start(Nodes::String)
            .with_ctx(
                Context {
                    leading: Some(&trivia),
                    ..Default::default()
                },
                parse(|s| s.token().with_mode(inner_string())),
            )
            .expect(Token::DoubleQuote)
            .end()
    })
}

fn interpolated<S: Sink >() -> impl Parser<StrToken, S> {
    parse(|s| {
             let leading = leading_trivia();
             let trailing = trailing_trivia();
        s.start(Nodes::Interpolated)
         .token()
         .parse(with_mode(with_ctx(Context {
             leading: Some(&leading),
             trailing: Some(&trailing),
         }, value())))
         .expect(StrToken::CloseI)
         .end()
    })
}

fn inner_string<S: Sink >() -> impl Parser<StrToken, S> {
    parse(|s| {
        s.start(Nodes::InnerString)
         .parse(repeated(|p| p
            .parse_else(parse(|s| s.alias(Nodes::InnerStringPart)))
            .at(StrToken::CloseI)
            .at(StrToken::Text)
            .parse(parse(|s| s.token()))
            .at(StrToken::OpenI)
            .parse(interpolated())
         , StrToken::Close))
        .end()
    })
}

fn unary<S: Sink >() -> impl Parser<Token, S> {
    parse(|s| {
        s.start(Nodes::Unary)
            .alias(Nodes::UnaryOp)
            .token()
            .parse(left_value())
            .end()
    })
}

fn boolean<S: Sink >() -> impl Parser<Token, S> {
    parse(|s| s.alias(Nodes::Boolean).token())
}

fn number<S: Sink >() -> impl Parser<Token, S> {
    parse(|s| s.alias(Nodes::Number).token())
}


pub(crate) fn leading_trivia<>() -> impl Trivia<Token> {
    trivia(|lexer| {
        while let Some(Token::LineEnd) = lexer.peek_token() {
            lexer.next();
        }
        while let Some(Token::Whitespace) | Some(Token::InlineComment) | Some(Token::BlockComment) =
            lexer.peek_token()
        {
            lexer.next();
        }
    })
}

pub(crate) fn trailing_trivia<>() -> impl Trivia<Token> {
    trivia(|lexer| {
        while let Some(Token::Whitespace) | Some(Token::InlineComment) | Some(Token::BlockComment) =
            lexer.peek_token()
        {
            lexer.next();
        }
    })
}

#[cfg(test)]
mod tests {
    use super::parser;
    use crate::lexers::neu::{Lexer, Token};
    use microtree_parser::{GreenSink, Spanned, State};

    #[test]
    fn lexer_tests() {
        test_runner::test_snapshots("neu", "lexer", |input| {
            let lexer = Lexer::new(input);

            let res: Vec<_> = lexer.map(|t: Spanned<Token>| format!("{:?}", t)).collect();
            format!("{:#?}", res)
        })
        .unwrap();
    }

    #[test]
    fn parser_tests() {
        test_runner::test_snapshots("neu", "parser", |input| {
            let lexer = Lexer::new(input);

            if std::env::var("DEBUG").is_ok() {
                let res: microtree_parser::TestSink = State::parse(lexer, parser());
                let res = res.events.join("\n");
                format!("{}", res)
            }
            else {
                let res: GreenSink = State::parse(lexer, parser());

                let root = res.finish().root;
                format!("{:?}", root)
            }
        })
        .unwrap();
    }
}
