use crate::common::separated;
use crate::markdown::inner_md_string;
use crate::{
    lexers::{neu::Token, string::Token as StrToken},
    Nodes,
};
use neu_parser::*;

pub fn parser() -> impl Parser<Token> {
    node(|builder| {
        builder.name(Nodes::Root);
        let trivia = trivia();
        let ctx = Context::new(&trivia);
        builder.parse_ctx(&ctx, value());
        builder.parse(token(None));
    })
}

pub(crate) fn value() -> impl Parser<Token> + Clone {
    let next = |state: &mut State<_>, ctx: &Context<_>| left_value().parse(state, ctx);
    Pratt::new(
        next,
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
            match op_token {
                Some(Token::OpDot) => {
                    builder.name(Nodes::IdentPath);
                }
                _ => {
                    builder.name(Nodes::Binary);
                }
            }
            builder.name(Nodes::Value);
            builder.parse(named(token(op_token), Nodes::Op));
        },
    )
    .parser()
}

fn left_value() -> impl Parser<Token> {
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
        builder.name(Nodes::Virtual);
        builder.name(Nodes::Value);
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
                builder.name(Nodes::Parens);
                builder.parse(token(Token::OpenP));
                builder.parse(value());
                builder.parse(token(Token::CloseP));
            })),
            _ => builder.parse(expected(VALUE_TOKENS)),
        };
    })
}

fn array() -> impl Parser<Token> {
    node(|builder| {
        builder.name(Nodes::Array);
        builder.parse(token(Token::OpenB));
        builder.parse(separated(value(), Token::Comma, Token::CloseB, true));
        builder.parse(token(Token::CloseB));
    })
}

fn identifier() -> impl Parser<Token> {
    named(token(Token::Identifier), Nodes::Identifier)
}

pub(crate) fn strukt_key() -> impl Parser<Token> {
    named(identifier(), Nodes::Key)
}

fn strukt() -> impl Parser<Token> {
    node(|builder| {
        builder.name(Nodes::Struct);
        builder.parse(token(Token::OpenC));
        builder.parse(separated(
            node(|builder| {
                builder.name(Nodes::Virtual);
                builder.parse(strukt_key());
                builder.parse(token(Token::OpAssign));
                builder.parse(value());
            }),
            Token::Comma,
            Token::CloseC,
            true,
        ));
        builder.parse(token(Token::CloseC));
    })
}

fn md_string() -> impl Parser<Token> {
    node(|builder| {
        builder.name(Nodes::Markdown);
        let ctx = Context::default();
        builder.parse_ctx(&ctx, token(Token::MdQuote));
        let ctx2 = Context::default();
        builder.parse_mode(&ctx2, inner_md_string());
        builder.parse_ctx(&ctx, token(Token::DoubleQuote));
    })
}

fn string() -> impl Parser<Token> {
    node(|builder: &mut NodeBuilder<'_, Token>| {
        builder.name(Nodes::String);
        let ctx = Context::default();
        builder.parse_ctx(&ctx, token(Token::DoubleQuote));
        let ctx2 = Context::default();
        builder.parse_mode(&ctx2, inner_string());
        builder.parse_ctx(&ctx, token(Token::DoubleQuote));
    })
}

fn inner_string() -> impl Parser<StrToken> {
    node(|builder| {
        builder.name(Nodes::Virtual);
        builder.name(Nodes::StrValue);
        loop {
            match builder.peek_token() {
                Some(StrToken::CloseI) | Some(StrToken::Text) => {
                    builder.parse(any_token());
                    continue;
                }
                Some(StrToken::OpenI) => {
                    builder.parse(node(|builder| {
                        builder.name(Nodes::Interpolated);
                        builder.parse(token(StrToken::OpenI));
                        let trivia = trivia();
                        let ctx = Context::new(&trivia);
                        builder.parse_mode(&ctx, value());
                        builder.parse(token(StrToken::CloseI));
                    }));
                }
                None | Some(StrToken::Close) => break,
            }
        }
    })
}

fn unary() -> impl Parser<Token> {
    node(|builder| {
        builder.name(Nodes::Unary);
        builder.parse(named(
            tokens(vec![Token::OpMinus, Token::OpBang, Token::OpDot]),
            Nodes::Op,
        ));
        builder.parse(value());
    })
}

fn boolean() -> impl Parser<Token> {
    named(tokens(vec![Token::True, Token::False]), Nodes::Boolean)
}

fn number() -> impl Parser<Token> {
    named(token(Token::Number), Nodes::Number)
}

pub(crate) fn trivia() -> impl Parser<Token> {
    node(|builder| {
        builder.name(Nodes::Trivia);
        while let Some(Token::Whitespace) | Some(Token::Comment) = builder.peek_token() {
            builder.next_token();
        }
    })
}

#[cfg(test)]
mod tests {
    use super::parser;
    use crate::lexers::neu::{Lexer, Token};
    use neu_parser::{Spanned, State};

    #[test]
    fn lexer_tests() {
        test_runner::test_snapshots("neu", "lexer", |input| {
            let lexer = Lexer::new(input);

            let res: Vec<_> = lexer
                .map(|t: Spanned<Token>| t.display(input, true).to_string())
                .collect();
            format!("{:#?}", res)
        })
        .unwrap();
    }

    #[test]
    fn parser_tests() {
        test_runner::test_snapshots("neu", "parser", |input| {
            let lexer = Lexer::new(input);

            let res = State::parse(lexer, parser());

            format!("{}", res.display(input))
        })
        .unwrap();
    }
}
