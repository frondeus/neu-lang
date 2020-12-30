use crate::TextRange;

#[derive(Debug, PartialEq, Eq)]
pub struct Error {
    desc: String,
    loc: TextRange,
}

impl Error {
    pub fn new(desc: impl ToString, loc: TextRange) -> Self {
        Self {
            desc: desc.to_string(),
            loc,
        }
    }
}
