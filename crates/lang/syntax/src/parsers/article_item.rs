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
        builder.name(Nodes::Markdown);
        builder.name(Nodes::Value);
        builder.parse(
            // Copierd from inner_md_string();
            node(|builder| {
                builder.name(Nodes::Virtual);
                builder.name(Nodes::Md_Value);
                let i = builder.state().lexer().input().clone();
                if let Some(BodyToken::Text) = builder.peek_token() {
                    markdown(builder, i);
                }
            })
        );
        //let i = builder.state().lexer().input().clone();
        //match builder.peek_token() {
            //None => (),
            //Some(BodyToken::Text) => {
                //markdown(builder, i);
            //}
        //}
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
