use crate::parsers::common::separated;
use crate::parsers::markdown::markdown;
use crate::{
    lexers::{
        article_item_body::Token as BodyToken, article_item_file::Token as FileToken,
        article_item_header::Token as HeaderToken, neu::Token as NeuToken,
    },
    parsers::neu, Nodes,
};
use neu_parser::*;
use crate::parsers::neu::{leading_trivia, trailing_trivia};

pub fn parser() -> impl Parser<FileToken> {
    node(|builder| {
        builder.name(Nodes::Root);
        match builder.peek_token() {
            None => (),
            Some(FileToken::ThreePlus) => {
                builder.parse(main_item());
            }
            Some(FileToken::Error) => {
                todo!("In theory this may be a classic md file so ignore it now.")
            }
        }
        builder.parse(token(None));
    })
}

fn main_item() -> impl Parser<FileToken> {
    node(|builder| {
        builder.name(Nodes::ArticleItem);
        builder.parse(token(FileToken::ThreePlus));
        let ctx = Context::default();
        builder.parse_mode(&ctx, main_item_header());
        let ctx = Context::default();
        builder.parse_mode(&ctx, main_item_body());
    })
}

fn main_item_body() -> impl Parser<BodyToken> {
    node(|builder| {
        builder.name(Nodes::ArticleBody);
        // This should be in inner parser
        loop {
            match builder.peek_token() {
                None => break,
                Some(BodyToken::Text) => {
                    builder.parse(node(|builder| {
                        let i = builder.state().lexer().input().clone();
                        builder.name(Nodes::Md_Value);
                        builder.name(Nodes::Value);
                        builder.name(Nodes::Markdown);
                        builder.name(Nodes::Virtual);
                        markdown(builder, i);
                    }));
                },
                Some(BodyToken::PlusPlus) => {
                    builder.parse(item());
                },
                Some(BodyToken::OpenBl) => {
                    builder.parse(item_bl());
                },
                Some(_) => {
                    builder.parse(expected(&[BodyToken::Text, BodyToken::PlusPlus, BodyToken::OpenBl]));
                }
            }
        }
    })
}

fn main_item_header() -> impl Parser<HeaderToken> {
    node(|builder| {
        builder.name(Nodes::Virtual);
        builder.parse(req_trivia(HeaderToken::InlineWhitespace));
        builder.parse(named(
            tokens(vec![HeaderToken::Identifier, HeaderToken::ItemId]),
            Nodes::Identifier,
        ));
        builder.parse(token(HeaderToken::Colon));
        builder.parse(named(token(HeaderToken::ItemId), Nodes::ArticleItemId));
        builder.parse(req_trivia(HeaderToken::InlineWhitespace));
        builder.parse(token(HeaderToken::ThreePlus));
        builder.parse(opt_ws());
        builder.parse(req_trivia(HeaderToken::NewLine));
        builder.parse(node(|builder| {
            builder.name(Nodes::Value);
            builder.name(Nodes::Struct);
            builder.parse(separated(
                node(|builder| {
                    builder.name(Nodes::Virtual);

                    let leading_trivia = leading_trivia();
                    let trailing_trivia = trailing_trivia();

                    let ctx = Context {
                        leading_trivia: Some(&leading_trivia),
                        trailing_trivia: Some(&trailing_trivia),
                    };
                    builder.parse_mode(&ctx, struct_key_val());
                }),
                HeaderToken::NewLine,
                HeaderToken::ThreePlus,
                true,
            ));
        }));
        builder.parse(token(HeaderToken::ThreePlus));
    })
}

fn item() -> impl Parser<BodyToken> {
    node(|builder| {
        builder.name(Nodes::ArticleItem);
        builder.parse(token(BodyToken::PlusPlus));
        let ctx = Context::default();
        builder.parse_mode(&ctx, item_header());
        let ctx = Context::default();
        builder.parse_mode(&ctx, item_body());
        builder.parse(token(BodyToken::PlusPlusEnd));
    })
}

fn item_header() -> impl Parser<HeaderToken> {
    node(|builder| {
        builder.name(Nodes::Virtual);
        builder.parse(req_trivia(HeaderToken::InlineWhitespace));
        builder.parse(named(
            tokens(vec![HeaderToken::Identifier, HeaderToken::ItemId]),
            Nodes::Identifier,
        ));
        builder.parse(token(HeaderToken::Colon));
        builder.parse(named(token(HeaderToken::ItemId), Nodes::ArticleItemId));
        builder.parse(req_trivia(HeaderToken::InlineWhitespace));
        builder.parse(token(HeaderToken::PlusPlus));
        builder.parse(opt_ws());
        builder.parse(req_trivia(HeaderToken::NewLine));
        builder.parse(node(|builder| {
            builder.name(Nodes::Value);
            builder.name(Nodes::Struct);
            builder.parse(separated(
                node(|builder| {
                    builder.name(Nodes::Virtual);

                    let leading_trivia = leading_trivia();
                    let trailing_trivia = trailing_trivia();

                    let ctx = Context {
                        leading_trivia: Some(&leading_trivia),
                        trailing_trivia: Some(&trailing_trivia),
                    };
                    builder.parse_mode(&ctx, struct_key_val());
                }),
                HeaderToken::NewLine,
                HeaderToken::ThreePlus,
                true,
            ));
        }));
        builder.parse(token(HeaderToken::ThreePlus));
    })
}

fn item_body() -> impl Parser<BodyToken> {
    node(|builder| {
        builder.name(Nodes::ArticleBody);
        builder.parse(
            node(|builder| {
                builder.name(Nodes::Virtual);
                let i = builder.state().lexer().input().clone();
                if let Some(BodyToken::Text) = builder.peek_token() {
                    builder.name(Nodes::Md_Value);
                    builder.name(Nodes::Markdown);
                    builder.name(Nodes::Value);
                    markdown(builder, i);
                }
            })
        );
    })
}

fn item_bl() -> impl Parser<BodyToken> {
    node(|builder| {
        builder.name(Nodes::ArticleRef);
        builder.parse(token(BodyToken::OpenBl));
        let ctx = Context::default();
        builder.parse_mode(&ctx, node(|builder| {
            builder.name(Nodes::Virtual);
            builder.parse(req_trivia(HeaderToken::InlineWhitespace));
            builder.parse(named(
                tokens(vec![HeaderToken::Identifier, HeaderToken::ItemId]),
                Nodes::Identifier,
            ));
            builder.parse(token(HeaderToken::Colon));
            builder.parse(named(token(HeaderToken::ItemId), Nodes::ArticleItemId));
            builder.parse(req_trivia(HeaderToken::InlineWhitespace));
        }));
        builder.parse(token(BodyToken::CloseBl));
    })
}

fn struct_key_val() -> impl Parser<NeuToken> {
    node(|builder| {
        builder.name(Nodes::Virtual);
        builder.parse(neu::strukt_key());
        builder.parse(token(NeuToken::OpAssign));
        builder.parse(neu::value());
    })
}

fn req_trivia(tok: HeaderToken) -> impl Parser<HeaderToken> {
    named(token(tok), Nodes::Trivia)
}

fn opt_ws() -> impl Parser<HeaderToken> {
    node(|builder| {
        builder.name(Nodes::Trivia);
        while let Some(HeaderToken::InlineWhitespace) = builder.peek_token() {
            builder.next_token();
        }
    })
}

#[cfg(test)]
mod tests {
    use super::parser;
    use crate::lexers::article_item_file::Lexer;
    use neu_parser::State;

    #[test]
    fn article_parser_tests() {
        test_runner::test_snapshots("md", "parser", |input| {
            let lexer = Lexer::new(input);

            let res = State::parse(lexer, parser());

            format!("{}", res.display(input))
        })
        .unwrap();
    }
}
