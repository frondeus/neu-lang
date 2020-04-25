pub mod core;

mod main_token;
mod main_lexer;

mod string_token;
mod string_lexer;

mod md_string_token {
    use derive_more::Display;
    use crate::core::TokenKind;

    #[derive(Debug, PartialEq, Clone, Copy, Display)]
    pub enum MdStrToken {
        #[display(fmt = "text")]
        Text,

        #[display(fmt = "`\"`")]
        Close,
    }

    impl TokenKind for MdStrToken {
        fn is_mergeable(self, other: Self) -> bool {
            match (self, other) {
                (Self::Text, Self::Text) => true,
                _ => false
            }
        }
    }

}
mod md_string_lexer {
    use crate::core::{LexerState, Lexer, Input, TextRange};
    use crate::MdStrToken;

    pub struct MdStringLexer(LexerState<MdStrToken>, usize);

    impl MdStringLexer {
        pub fn set_hash(&mut self, hash: usize) {
            self.1 = hash;
        }
    }

    impl Lexer for MdStringLexer {
        type Token = MdStrToken;

        fn build(state: LexerState<MdStrToken>) -> Self { Self(state, 0) }
        fn state_mut(&mut self) -> &mut LexerState<Self::Token> { &mut self.0 }
        fn state(&self) -> &LexerState<Self::Token> { &self.0 }

        fn lex(&mut self) -> Option<(Self::Token, TextRange)> {
            let hash = self.1;
            let input = self.input_mut();
            let i = input.as_ref();
            if i.is_empty() { return None; }
            let pat = format!("{:#<width$}", "\"", width = hash + 1);
            if i.starts_with(&pat) {
                return Some((MdStrToken::Close, input.chomp(pat.len())));
            }

            Some((MdStrToken::Text, input.chomp(1)))
        }
    }
}

mod nodes;

pub use crate::main_token::*;
pub use crate::main_lexer::*;

pub use crate::string_token::*;
pub use crate::string_lexer::*;

pub use crate::md_string_token::*;
pub use crate::md_string_lexer::*;

pub use crate::nodes::*;

pub mod neu;
pub mod markdown;
