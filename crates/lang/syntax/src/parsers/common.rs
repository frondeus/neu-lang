use crate::Nodes;
use neu_parser::*;

pub fn separated<Token: TokenKind + 'static>(
    parser: impl Parser<Token> + Clone,
    separator: Token,
    close_token: Token,
    trailing: bool,
) -> impl Parser<Token> {
    node(move |builder| {
        builder.name(Nodes::Virtual);
        match builder.peek_token() {
            None => (),
            Some(tok) if tok == close_token => (),
            _ => 'outer: loop {
                builder.parse(parser.clone());
                'inner: loop {
                    match builder.peek_token() {
                        None => break 'outer,
                        Some(tok) if tok == close_token => break 'outer,
                        Some(tok) if tok == separator => {
                            builder.parse(token(separator)); //recover(","));
                            match builder.peek_token() {
                                Some(tok) if trailing && tok == close_token => break 'outer,
                                _ => break 'inner,
                            }
                        }
                        _ => {
                            builder.parse(tokens(vec![separator, close_token]));
                        }
                    }
                }
            },
        }
    })
}
