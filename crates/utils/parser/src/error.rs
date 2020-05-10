use std::fmt;
use crate::{Spanned, TokenKind};

#[derive(Clone)]
pub enum Error<Tok: TokenKind> {
    Expected { found: Option<Spanned<Tok>>, expected: Vec<Tok> },
    ExpectedEOF { found: Spanned<Tok> },
}

impl<Tok: TokenKind> Error<Tok> {
    pub fn display<'a, 's>(&'a self, str: &'s str) -> DisplayError<'a, 's, Tok> {
        DisplayError { error: self, str }
    }
}

impl<Tok: TokenKind> fmt::Debug for Error<Tok> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExpectedEOF { found } => write!(f, r#""Expected EOF but found {:?}""#, found),
            Self::Expected { found: None, expected } => {
                format_expected(&expected, f)?;
                write!(f, " but found EOF")
            }
            Self::Expected { found: Some(found), expected } => {
                format_expected(&expected, f)?;
                write!(f, " but found {:?}", found)
            }
        }
    }
}

fn format_expected<Tok: TokenKind>(expected: &[Tok], f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

pub struct DisplayError<'a, 's, Tok: TokenKind> {
    str: &'s str,
    error: &'a Error<Tok>
}

impl<'a, 's, Tok: TokenKind> fmt::Display for DisplayError<'a, 's, Tok> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.error {
            Error::ExpectedEOF { found } => write!(f, r#"Expected EOF but found {}"#, found.display(self.str, false)),
            Error::Expected { found: None, expected } => {
                format_expected(&expected, f)?;
                write!(f, " but found EOF")
            }
            Error::Expected { found: Some(found), expected } => {
                format_expected(&expected, f)?;
                write!(f, " but found {}", found.display(self.str, false))
            }
        }
    }
}

