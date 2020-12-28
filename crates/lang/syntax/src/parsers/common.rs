use crate::Nodes;
use microtree_parser::*;
use microtree_parser::parsers::*;

pub fn separated<'s, Token: TokenKind<'s> + 'static>(
    parser: impl Parser<'s, Token> + Clone,
    separator: Token,
    close_token: Token,
    trailing: bool,
) -> impl Parser<'s, Token> {
    node(move |builder| {
        let mut builder = builder.node();
        match builder.peek_token() {
            None => (),
            Some(tok) if tok == close_token => (),
            _ => 'outer: loop {
                builder = builder.parse(parser.clone());
                'inner: loop {
                    match builder.peek_token() {
                        None => break 'outer,
                        Some(tok) if tok == close_token => break 'outer,
                        Some(tok) if tok == separator => {
                            builder = builder.parse(token(separator)); //recover(","));
                            match builder.peek_token() {
                                Some(tok) if trailing && tok == close_token => break 'outer,
                                _ => break 'inner,
                            }
                        }
                        _ => {
                            builder = builder.parse(tokens(vec![separator, close_token]));
                        }
                    }
                }
            },
        }
        builder.finish()
    })
}
