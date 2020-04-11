use crate::Token;
use std::fmt;
use crate::core::Spanned;

#[derive(Clone)]
pub enum Error {
    UnexpectedToken { found: Spanned<Token>, expected: Vec<Token> },
    UnexpectedEOF { expected: Vec<Token> },
    ExpectedEOF { found: Spanned<Token> },
}

impl Error {
    pub fn display<'a, 's>(&'a self, str: &'s str) -> DisplayError<'a, 's> {
        DisplayError { error: self, str }
    }
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

fn format_expected(expected: &[Token], f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Expected ")?;
    let last = expected.len() - 1;
    if last > 0 {
        write!(f, "one of ")?;
    }
    let iter = expected.iter();
    for (i, token) in iter.enumerate() {
        if i == 0 {
            write!(f, "{}", token)?;
        } else {
            write!(f, ", {}", token)?;
        }
    }
    Ok(())
}

pub struct DisplayError<'a, 's> {
    str: &'s str,
    error: &'a Error
}

impl<'a, 's> fmt::Display for DisplayError<'a, 's> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.error {
            Error::ExpectedEOF { found } => write!(f, r#"Expected EOF but found {}"#, found.display(self.str, false)),
            Error::UnexpectedEOF { expected } => {
                format_expected(&expected, f)?;
                write!(f, " but found EOF")
            }
            Error::UnexpectedToken { found, expected } => {
                format_expected(&expected, f)?;
                write!(f, " but found {}", found.display(self.str, false))
            }
        }
    }
}

