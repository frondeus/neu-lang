use microtree_parser::{TokenKind, Source};

#[derive(Debug, PartialEq, Clone, Copy, TokenKind)]
#[token_kind(mergeable = "mergeable")]
pub enum Token {
    #[token_kind(token = "++ end ++")]
    PlusPlusEnd,

    #[token_kind(token = "++")]
    PlusPlus,

    #[token_kind(token = "[+")]
    OpenBl,

    #[token_kind(token = "+]")]
    CloseBl,

    #[token_kind(error, display = "text")]
    Text,
}

pub type Lexer<'s, T = Token> = microtree_parser::Lexer<'s, T>;

fn mergeable(first: Token, other: Token) -> bool {
    match (first, other) {
        (Token::Text, Token::Text) => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexer_test() {
        let input = r#"\n\nAla ma kota"#;

        let lexer: Lexer<'_, Token> = Lexer::new(input);
        let tokens: Vec<_> = lexer.collect();
        dbg!(&tokens);
        todo!();
    }
}
