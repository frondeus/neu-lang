use derive_more::Display;

#[derive(Debug, PartialEq, Clone, Copy, Display)]
pub enum Token {
    #[display(fmt = "error")]
    Error,

    #[display(fmt = "` `, `\t`, `\n`")]
    Whitespace,

    #[display(fmt = "comment")]
    Comment,

    #[display(fmt = "number")]
    Number,

    #[display(fmt = "`true`")]
    True,

    #[display(fmt = "`false`")]
    False,

    #[display(fmt = "`-`")]
    OpMinus,

    #[display(fmt = "`!`")]
    OpBang,

    #[display(fmt = "`+`")]
    OpPlus,

    #[display(fmt = "`*`")]
    OpStar,

    #[display(fmt = "`/`")]
    OpSlash,

    #[display(fmt = "`==`")]
    OpDEqual,

    #[display(fmt = "`=`")]
    OpAssign,

    #[display(fmt = "identifier")]
    Identifier,

    #[display(fmt = "`(`")]
    OpenP,

    #[display(fmt = "`)`")]
    CloseP,

    #[display(fmt = "`{{`")]
    OpenC,

    #[display(fmt = "`\"`")]
    DoubleQuote,

    #[display(fmt = "`md`")]
    MdQuote,

    #[display(fmt = "`}}`")]
    CloseC,

    #[display(fmt = "`[`")]
    OpenB,

    #[display(fmt = "`]`")]
    CloseB,

    #[display(fmt = "`,`")]
    Comma,

    #[display(fmt = "`.`")]
    OpDot
}

