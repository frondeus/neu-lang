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
use crate::parsers::markdown::markdown;

pub fn parser<S: Sink>() -> impl Parser<FileToken, S> {
    parse(|s| {
        s.peek()
            .at(None)
            .at_unexpected(FileToken::Error)
            .parse(skip())
            .at(FileToken::ThreePlus)
            .parse(main_item())
            .expect()
            .expect(None)
    })
}

fn main_item<S: Sink>() -> impl Parser<FileToken, S> {
    parse(|s| {
        s
            .alias(Nodes::MainArticle)
            .start(Nodes::ArticleItem)
            .with_mode(main_item_header())
            .with_mode(main_item_body())
        .end()
    })
}

fn main_item_body<S: Sink>() -> impl Parser<BodyToken, S> {
    item_body(false)
}

fn main_item_header<S: Sink>() -> impl Parser<HeaderToken, S> {
    parse(|s| {
        s.start(Nodes::ArticleHeader)
        .expect(HeaderToken::ThreePlus)
        .parse(req_trivia(HeaderToken::InlineWhitespace))
        .alias(Nodes::Identifier)
        .expect(HeaderToken::Identifier)
        .expect(HeaderToken::Colon)
        .alias(Nodes::ArticleItemId)
        .expect(HeaderToken::ItemId)
        .parse(req_trivia(HeaderToken::InlineWhitespace))
        .expect(HeaderToken::ThreePlus)
        .parse(opt_ws())
        .parse(req_trivia(HeaderToken::NewLine))
        .parse(parse(|s| {
            let leading_trivia = leading_trivia();
            let trailing_trivia = trailing_trivia();
            s.alias(Nodes::Value)
            .start(Nodes::Strukt)
            .parse(separated(
                with_mode(with_ctx(Context {
                            leading: Some(&leading_trivia),
                            trailing: Some(&trailing_trivia),
                }, struct_key_val())),
                HeaderToken::NewLine,
                HeaderToken::ThreePlus,
                true
            ))
            .end()
        }))
        .end()
    })
}

fn item<S: Sink>() -> impl Parser<BodyToken, S> {
    parse(|s| {
            s.alias(Nodes::SubArticle)
            .start(Nodes::ArticleItem)
            .with_mode(item_header())
            .with_mode(item_body(true))
            .expect(BodyToken::PlusPlusEnd)
        .end()
    })
}

fn item_header<S: Sink>() -> impl Parser<HeaderToken, S> {
    parse(|s| {
        s.start(Nodes::SubArticleHeader)
        .expect(HeaderToken::PlusPlus)
        .parse(req_trivia(HeaderToken::InlineWhitespace))
        .alias(Nodes::Identifier)
        .expect(HeaderToken::Identifier)
        .expect(HeaderToken::Colon)
        .alias(Nodes::ArticleItemId)
        .expect(HeaderToken::ItemId)
        .parse(req_trivia(HeaderToken::InlineWhitespace))
        .expect(HeaderToken::PlusPlus)
        .parse(opt_ws())
        .parse(req_trivia(HeaderToken::NewLine))
        .parse(parse(|s| {
            let leading_trivia = leading_trivia();
            let trailing_trivia = trailing_trivia();
            s.alias(Nodes::Value)
             .start(Nodes::Strukt)
             .parse(separated(
                 with_mode(with_ctx(Context {
                     leading: Some(&leading_trivia),
                     trailing: Some(&trailing_trivia),
                 }, struct_key_val())),
                 HeaderToken::NewLine,
                 HeaderToken::ThreePlus,
                 true,
             ))
             .end()
        }))
        .end()
    })
}

fn item_peek_end<'c, 's, S: Sink>(s: Builder<'c, 's, BodyToken ,S>, ends: bool) ->  Peek<'c, 's, BodyToken, S> {
    if ends {
        s.peek().at(BodyToken::PlusPlusEnd)
            .at_unexpected(None)
    } else {
        s.peek().at(None)
    }
}

fn item_body<S: Sink>(ends: bool) -> impl Parser<BodyToken, S> {
    parse(move |s| {
        // We are not using repeated because of this peek_end behavior
        let p = item_peek_end(s.start(Nodes::ArticleBody).expect(BodyToken::ThreePlus), ends);
        match p {
            Peek::Found { s, .. } => s,
            Peek::None { mut s, .. } => loop {
                let p = item_peek_end(s, ends);
                s = match p {
                    Peek::Found { s, .. } => break s,
                    p => p
                        .at(BodyToken::Text).parse(
                            parse(|s| s
                                  .start(Nodes::Markdown)
                                  .parse(markdown())
                                  .end())
                        )
                        .at(BodyToken::PlusPlus).parse(item())
                        .at(BodyToken::OpenBl).parse(item_bl())
                        .expect()
                }
            }
        }
        .end()
    })
}

fn item_bl<S: Sink>() -> impl Parser<BodyToken, S> {
    parse(|s| {
        s.start(Nodes::ArticleRef)
        .expect(BodyToken::OpenBl)
        .with_mode(
            parse(|s| {
                s
                .parse(req_trivia(HeaderToken::InlineWhitespace))
                .alias(Nodes::Identifier)
                .expect(HeaderToken::Identifier)
                .expect(HeaderToken::Colon)
                .alias(Nodes::ArticleItemId)
                .expect(HeaderToken::ItemId)
                .parse(req_trivia(HeaderToken::InlineWhitespace))
            })
        )
        .expect(BodyToken::CloseBl)
        .end()
    })
}

fn struct_key_val<S: Sink>() -> impl Parser<NeuToken, S> + Clone {
    parse(|s| {
        s.parse(neu::strukt_key())
        .expect(NeuToken::OpAssign)
        .parse(neu::value())
    })
}

fn req_trivia<S: Sink>(tok: HeaderToken) -> impl Parser<HeaderToken, S> {
    parse(move |s| s.expect(tok))
}

fn opt_ws<S: Sink>() -> impl Parser<HeaderToken, S> {
    parse(|mut s| {
        loop {
            s = match s.peek()
                   .at(HeaderToken::InlineWhitespace) {
                       Peek::Found { s, .. } => s.token(),
                       Peek::None { s, .. } => break s
                   }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::parser;
    use crate::lexers::article_item_file::Lexer;
    use microtree_parser::{GreenSink, State};

    #[test]
    fn article_parser_tests() {
        test_runner::test_snapshots("md", "parser", |input| {
            let lexer = Lexer::new(input);

            if std::env::var("DEBUG").is_ok() {
                let res: microtree_parser::TestSink = State::parse(lexer, parser());
                let res = res.events.join("\n");
                format!("{}", res)
            }
            else {
                let res: GreenSink = State::parse(lexer, parser());
                let res = res.finish();

                format!("{:?}", res.root)
            }
        })
        .unwrap();
    }
}
