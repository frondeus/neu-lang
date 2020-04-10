use crate::Token;
use std::fmt;

#[derive(Clone)]
pub enum Error {
    UnexpectedToken { found: Token, expected: Vec<Token> },
    UnexpectedEOF { expected: Vec<Token> },
    ExpectedEOF { found: Token },
}

fn format_expected(expected: &[Token], f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Expected ")?;
    let last = expected.len() - 1;
    if last > 0 {
        write!(f, "one of ")?;
    }
    let iter = expected.iter();
    for (i, token) in iter.enumerate() {
        if i == 0 {
            write!(f, "{:?}", token)?;
        } else {
            write!(f, ", {:?}", token)?;
        }
    }
    Ok(())
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExpectedEOF { found } => write!(f, r#""Expected EOF but found {:?}""#, found),
            Self::UnexpectedEOF { expected } => {
                format_expected(&expected, f)?;
                write!(f, " but found EOF")
            }
            Self::UnexpectedToken { found, expected } => {
                format_expected(&expected, f)?;
                write!(f, " but found {:?}", found)
            }
        }
    }
}
