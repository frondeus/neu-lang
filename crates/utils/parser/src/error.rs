use crate::{Spanned, TokenKind};
use std::fmt;
pub(crate) use neu_diagnostics::ToReport;

#[derive(Clone)]
pub enum ParseError<Tok: TokenKind> {
    Expected {
        found: Option<Spanned<Tok>>,
        expected: Vec<Tok>,
    },
    ExpectedEOF {
        found: Spanned<Tok>,
    },
}

impl<Tok: TokenKind> ToReport for ParseError<Tok> {
    fn to_report(&self, str: &str) -> String {
        DisplayError { error: self, str }.to_string()
    }
}

impl<Tok: TokenKind> fmt::Debug for ParseError<Tok> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExpectedEOF { found } => write!(f, r#""Expected EOF but found {:?}""#, found),
            Self::Expected {
                found: None,
                expected,
            } => {
                format_expected(&expected, f)?;
                write!(f, " but found EOF")
            }
            Self::Expected {
                found: Some(found),
                expected,
            } => {
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
    error: &'a ParseError<Tok>,
}

impl<'a, 's, Tok: TokenKind> fmt::Display for DisplayError<'a, 's, Tok> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.error {
            ParseError::ExpectedEOF { found } => write!(
                f,
                r#"Expected EOF but found {}"#,
                found.display(self.str, false)
            ),
            ParseError::Expected {
                found: None,
                expected,
            } => {
                format_expected(&expected, f)?;
                write!(f, " but found EOF")
            }
            ParseError::Expected {
                found: Some(found),
                expected,
            } => {
                format_expected(&expected, f)?;
                write!(f, " but found {}", found.display(self.str, false))
            }
        }
    }
}
