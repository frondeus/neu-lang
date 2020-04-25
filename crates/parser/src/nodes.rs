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
    Md_Rule,
    Md_BlockQuote,
    Md_UnorderedList,
    Md_ListItem,
    Md_OrderedList,

    Md_Link,
    Md_InlineLink,
    Md_ReferenceLink,
    Md_ShortcutLink,
    Md_AutoLink,
    Md_EmailLink,

    Md_Image,
    Md_InlineImage,
    Md_ReferenceImage,
    Md_ShortcutImage,
    Md_AutoImage,
    Md_EmailImage,

    Md_LinkUrl,
    Md_LinkTitle,

    Md_ImageSrc,
    Md_ImageTitle,

    Md_CodeBlock,
    Md_CodeBlockLang,

    Md_H1,
    Md_H2,
    Md_H3,
    Md_H4,
    Md_H5,
    Md_H6,

    Md_Text,
    Md_Html
}
