use crate::core::Name;
use crate::nodes;

nodes! {
    Value,
    Parens,
    Number,
    Boolean,
    String,
    Interpolated,


    StrValue,

    Unary,
    Binary,
    Op,

    Struct,
    Identifier,
    Key,

    Array,
    IdentPath,

    Markdown,
    Md_Value,

    Md_Paragraph,
    Md_Emphasis,
    Md_Strong,
    Md_SoftBreak,

    Md_H1,
    Md_H2,
    Md_H3,
    Md_H4,
    Md_H5,
    Md_H6,

    Md_Text,
    Md_Html
}
