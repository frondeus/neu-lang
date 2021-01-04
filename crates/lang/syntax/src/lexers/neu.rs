use crate::HashCount;
use microtree_parser::{TextSize, Source, TokenKind, CallbackResult};

fn lex_comment(_chomped: TextSize, source: &mut Source<'_>, extras: &mut HashCount) -> bool {
    source.as_ref()
        .find("*/")
        .map(|i| source.chomp(i + 2))
        .is_some()
}

fn lex_plus(_chomped: TextSize, source: &mut Source<'_>, extras: &mut HashCount) -> bool {
    !source.as_ref().starts_with("+")
}

fn lex_dquote(_chomped: TextSize, source: &mut Source<'_>, extras: &mut HashCount) -> bool {
    if extras.count > 0 {
        let hash_count =extras.count;
        let hash = "#".repeat(hash_count);
        if source.as_ref().starts_with(&hash) {
            source.chomp(hash_count);
            extras.count = 0;
            true
        }
        else {
            false
        }
    } else {
        true
    }
}

fn lex_mdquote(_chomped: TextSize, source: &mut Source<'_>, extras: &mut HashCount) -> bool {
    let mut remainder = source.as_ref();
    let mut hash = 0;
    while remainder.starts_with("#") {
        hash += 1;
        source.chomp(1);
        remainder = source.as_ref();
    }
    extras.count = hash;
    let quote = remainder.starts_with("\"");
    if quote {
        source.chomp(1);
    }
    quote
}

#[derive(Debug, PartialEq, Clone, Copy, TokenKind)]
#[token_kind(extras = "HashCount", mergeable = "mergeable")]
pub enum Token {
    #[token_kind(regex = r"[ \t]+", display = r"` `, `\t`")]
    Whitespace,

    #[token_kind(regex = r"(\r?\n)+", display = r"`\n`, `\r\n`")]
    LineEnd,

    #[token_kind(regex = r"//[^\n]*", display = "//")]
    InlineComment,

    #[token_kind(token = "/*", callback = "lex_comment", display = "/*")]
    BlockComment,

    #[token_kind(regex = r"[0-9]+")]
    Number,

    #[token_kind(token = "true")]
    True,

    #[token_kind(token = "false")]
    False,

    #[token_kind(token = "-")]
    OpMinus,

    #[token_kind(token = "!")]
    OpBang,

    #[token_kind(token = "+", callback = "lex_plus")]
    OpPlus,

    #[token_kind(token = "*")]
    OpStar,

    #[token_kind(token = "/")]
    OpSlash,

    #[token_kind(token = "==")]
    OpDEqual,

    #[token_kind(token = "=")]
    OpAssign,

    #[token_kind(regex = r"[a-zA-Z_]+[a-zA-Z_0-9]*", display = "identifier")]
    Identifier,

    #[token_kind(token = "(")]
    OpenP,

    #[token_kind(token = ")")]
    CloseP,

    #[token_kind(token = "{", display = "`{{`")]
    OpenC,

    #[token_kind(token = "\"", callback = "lex_dquote")]
    DoubleQuote,

    #[token_kind(token = "md", callback = "lex_mdquote")]
    MdQuote,

    #[token_kind(token = "}", display = "`}}`")]
    CloseC,

    #[token_kind(token = "[")]
    OpenB,

    #[token_kind(token = "]")]
    CloseB,

    #[token_kind(token = ",")]
    Comma,

    #[token_kind(token = ".")]
    OpDot,

    #[token_kind(error)]
    Error,

}

pub type Lexer<'s, T = Token> = microtree_parser::Lexer<'s, T>;

fn mergeable(first: Token, other: Token) -> bool {
    match (first, other) {
        (Token::Error, Token::Error) => true,
        (Token::LineEnd, Token::LineEnd) => true,
        _ => false,
    }
}
