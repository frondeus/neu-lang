use derive_more::Display;
use crate::core::TokenKind;

#[derive(Debug, PartialEq, Clone, Copy, Display)]
pub enum StrToken {
    #[display(fmt = "text")]
    Text,

    #[display(fmt = "`${{`")]
    OpenI,

    #[display(fmt = "`}}`")]
    CloseI,

    #[display(fmt = "`\"`")]
    Close,

}

impl TokenKind for StrToken {
    fn is_mergeable(self, other: Self) -> bool {
        match (self, other) {
            (Self::Text, Self::Text) => true,
            (Self::Text, Self::CloseI) => true,
            _ => false
        }
    }
}

