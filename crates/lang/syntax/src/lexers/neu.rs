use crate::HashCount;
use derive_more::Display;
use microtree_parser::TokenKind;
use logos::Logos;

#[derive(Debug, PartialEq, Clone, Copy, Display, Logos)]
#[logos(extras = HashCount)]
pub enum Token {
    #[display(fmt = "error")]
    #[error]
    Error,

    #[display(fmt = "` `, `\t`")]
    #[regex(r#"[ \t]+"#)]
    Whitespace,

    #[display(fmt = "`\n`, `\r\n`")]
    #[regex(r#"(\r?\n)+"#)]
    LineEnd,

    #[display(fmt = "inline comment")]
    #[regex(r#"//[^\n]*"#)]
    InlineComment,

    #[display(fmt = "block comment")]
    #[token("/*", |lex| {
        lex.remainder()
           .find("*/")
           .map(|i| lex.bump(i + 2))
           .is_some()
    })]
    BlockComment,

    #[display(fmt = "number")]
    #[regex(r#"[0-9]+"#)]
    Number,

    #[display(fmt = "`true`")]
    #[token("true")]
    True,

    #[display(fmt = "`false`")]
    #[token("false")]
    False,

    #[display(fmt = "`-`")]
    #[token("-")]
    OpMinus,

    #[display(fmt = "`!`")]
    #[token("!")]
    OpBang,

    #[display(fmt = "`+`")]
    #[token("+", |lex| {
        !lex.remainder().starts_with("+") // Collision with `+++`
    })]
    OpPlus,

    #[display(fmt = "`*`")]
    #[token("*")]
    OpStar,

    #[display(fmt = "`/`")]
    #[token("/")]
    OpSlash,

    #[display(fmt = "`==`")]
    #[token("==")]
    OpDEqual,

    #[display(fmt = "`=`")]
    #[token("=")]
    OpAssign,

    #[display(fmt = "identifier")]
    #[regex("[a-zA-Z_]+[a-zA-Z_0-9]*")]
    Identifier,

    #[display(fmt = "`(`")]
    #[token("(")]
    OpenP,

    #[display(fmt = "`)`")]
    #[token(")")]
    CloseP,

    #[display(fmt = "`{{`")]
    #[token("{")]
    OpenC,

    #[display(fmt = "`\"`")]
    #[token("\"", |lex| {
        if lex.extras.count > 0 {
            let hash_count = lex.extras.count;
            let hash = "#".repeat(hash_count);
            if lex.remainder().starts_with(&hash) {
                lex.bump(hash_count);
                lex.extras.count = 0;
                true
            }
            else {
                false
            }
        } else {
            true
        }
    })]
    DoubleQuote,

    #[display(fmt = "`md\"`")]
    #[token("md", |lex| {
        let mut remainder = lex.remainder();
        let mut hash = 0;
        while remainder.starts_with("#") {
            hash += 1;
            lex.bump(1);
            remainder = lex.remainder();
        }
        lex.extras.count = hash;
        let quote = remainder.starts_with("\"");
        if quote {
            lex.bump(1);
        }
        quote
    })]
    MdQuote,

    #[display(fmt = "`}}`")]
    #[token("}")]
    CloseC,

    #[display(fmt = "`[`")]
    #[token("[")]
    OpenB,

    #[display(fmt = "`]`")]
    #[token("]")]
    CloseB,

    #[display(fmt = "`,`")]
    #[token(",")]
    Comma,

    #[display(fmt = "`.`")]
    #[token(".")]
    OpDot,
}

pub type Lexer<'s, T = Token> = microtree_parser::Lexer<'s, T>;

impl<'s> TokenKind<'s> for Token {
    fn mergeable(self, other: Self) -> bool {
        match (self, other) {
            (Self::Error, Self::Error) => true,
            (Self::LineEnd, Self::LineEnd) => true,
            _ => false,
        }
    }
}
