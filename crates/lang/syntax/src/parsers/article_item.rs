use crate::parsers::common::separated;
//use crate::parsers::markdown::markdown;
use crate::parsers::neu::{leading_trivia, trailing_trivia};
use crate::{
    lexers::{
        article_item_body::Token as BodyToken, article_item_file::Token as FileToken,
        article_item_header::Token as HeaderToken, neu::Token as NeuToken,
    },
    parsers::neu,
    Nodes,
};
use microtree_parser::*;
use microtree_parser::parsers::*;

pub fn parser<'s>() -> impl Parser<'s, FileToken> {
    node(|builder| {
        let mut builder = builder.node().name(Nodes::Root);
        match builder.peek_token() {
            None => (),
            Some(FileToken::ThreePlus) => {
                builder = builder.parse(main_item());
            }
            Some(FileToken::Error) => { }
        }
        builder
            .parse(token(None))
            .finish()
    })
}

fn main_item<'s>() -> impl Parser<'s, FileToken> {
    node(|builder| {
        builder.node()
        .name(Nodes::ArticleItem)
        .parse(token(FileToken::ThreePlus))
        .parse_mode(main_item_header(), None)
        .parse_mode(main_item_body(), None)
        .finish()
    })
}

fn main_item_body<'s>() -> impl Parser<'s, BodyToken> {
    item_body(false)
}

fn main_item_header<'s>() -> impl Parser<'s,HeaderToken> {
    node(|builder| {
        builder.node()
        .parse(req_trivia(HeaderToken::InlineWhitespace))
        .parse(named(
            tokens(vec![HeaderToken::Identifier, HeaderToken::ItemId]),
            Nodes::Identifier,
        ))
        .parse(token(HeaderToken::Colon))
        .parse(named(token(HeaderToken::ItemId), Nodes::ArticleItemId))
        .parse(req_trivia(HeaderToken::InlineWhitespace))
        .parse(token(HeaderToken::ThreePlus))
        .parse(opt_ws())
        .parse(req_trivia(HeaderToken::NewLine))
        .parse(node(|builder| {
            builder.node()
            .name(Nodes::Value)
            .name(Nodes::Strukt)
            .parse(separated(
                node(|builder| {
                    let leading_trivia = leading_trivia();
                    let trailing_trivia = trailing_trivia();

                    let ctx = Context {
                        leading_trivia: Some(&leading_trivia),
                        trailing_trivia: Some(&trailing_trivia),
                    };
                    builder
                        .parse_mode(struct_key_val(), &ctx)
                }),
                HeaderToken::NewLine,
                HeaderToken::ThreePlus,
                true,
            ))
            .finish()
        }))
        .parse(token(HeaderToken::ThreePlus))
        .finish()
    })
}

fn item<'s>() -> impl Parser<'s, BodyToken> {
    node(|builder| {
        builder.node().
            name(Nodes::ArticleItem)
            .parse(token(BodyToken::PlusPlus))
            .parse_mode(item_header(), None)
            .parse_mode(item_body(true), None)
            .parse(token(BodyToken::PlusPlusEnd))
            .finish()
    })
}

fn item_header<'s>() -> impl Parser<'s, HeaderToken> {
    node(|builder| {
        builder.node()
        .parse(req_trivia(HeaderToken::InlineWhitespace))
        .parse(named(
            tokens(vec![HeaderToken::Identifier, HeaderToken::ItemId]),
            Nodes::Identifier,
        ))
        .parse(token(HeaderToken::Colon))
        .parse(named(token(HeaderToken::ItemId), Nodes::ArticleItemId))
        .parse(req_trivia(HeaderToken::InlineWhitespace))
        .parse(token(HeaderToken::PlusPlus))
        .parse(opt_ws())
        .parse(req_trivia(HeaderToken::NewLine))
        .parse(node(|builder| {
            builder.node()
            .name(Nodes::Value)
            .name(Nodes::Strukt)
            .parse(separated(
                node(|builder| {
                    let leading_trivia = leading_trivia();
                    let trailing_trivia = trailing_trivia();

                    let ctx = Context {
                        leading_trivia: Some(&leading_trivia),
                        trailing_trivia: Some(&trailing_trivia),
                    };
                    builder.parse_mode(struct_key_val(), &ctx)
                }),
                HeaderToken::NewLine,
                HeaderToken::ThreePlus,
                true,
            ))
            .finish()
        }))
        .parse(token(HeaderToken::ThreePlus))
        .finish()
    })
}

fn item_body<'s>(ends: bool) -> impl Parser<'s, BodyToken> {
    node(move |builder| {
        let mut builder = builder.node()
            .name(Nodes::ArticleBody);
        loop {
            match builder.peek_token() {
                None => break builder.finish(),
                Some(BodyToken::PlusPlusEnd) if ends => break builder.finish(),
                Some(BodyToken::Text) => {
                    builder = builder.parse(node(|builder| {
                        //let i = builder.state().lexer().input().clone();
                        //builder.name(Nodes::Md_Value);
                        //builder.name(Nodes::Value);
                        //builder.name(Nodes::Markdown);
                        //builder.name(Nodes::Virtual);
                        //markdown(builder, i);
                        builder.none()
                    }));
                }
                Some(BodyToken::PlusPlus) => {
                    builder = builder.parse(item());
                }
                Some(BodyToken::OpenBl) => {
                    builder = builder.parse(item_bl());
                }
                Some(_) => {
                    builder = builder.parse(tokens(vec![
                        BodyToken::Text,
                        BodyToken::PlusPlus,
                        BodyToken::OpenBl,
                    ]));
                }
            }
        }
    })
}

fn item_bl<'s>() -> impl Parser<'s, BodyToken> {
    node(|builder| {
        builder.node()
        .name(Nodes::ArticleRef)
        .parse(token(BodyToken::OpenBl))
        .parse_mode(
            node(|builder| {
                builder.node()
                .parse(req_trivia(HeaderToken::InlineWhitespace))
                .parse(named(
                    tokens(vec![HeaderToken::Identifier, HeaderToken::ItemId]),
                    Nodes::Identifier,
                ))
                .parse(token(HeaderToken::Colon))
                .parse(named(token(HeaderToken::ItemId), Nodes::ArticleItemId))
                .parse(req_trivia(HeaderToken::InlineWhitespace))
                .finish()
            }),
            None
        )
        .parse(token(BodyToken::CloseBl))
        .finish()
    })
}

fn struct_key_val<'s>() -> impl Parser<'s, NeuToken> {
    node(|builder| {
        builder.node()
        .parse(neu::strukt_key())
        .parse(token(NeuToken::OpAssign))
        .parse(neu::value())
        .finish()
    })
}

fn req_trivia<'s>(tok: HeaderToken) -> impl Parser<'s, HeaderToken> {
    token(tok)
}

fn opt_ws<'s>() -> impl Parser<'s, HeaderToken> {
    node(|builder| {
        let mut builder = builder.node();
        while let Some(HeaderToken::InlineWhitespace) = builder.peek_token() {
            builder = builder.parse(any_token());
        }
        builder.finish()
    })
}

#[cfg(test)]
mod tests {
    use super::parser;
    use crate::lexers::article_item_file::Lexer;
    use microtree_parser::{ParseResult, State};

    #[test]
    fn article_parser_tests() {
        test_runner::test_snapshots("md", "parser", |input| {
            let lexer = Lexer::new(input);

            let res: ParseResult = State::parse(lexer, parser());

            format!("{:?}", res)
        })
        .unwrap();
    }
}
