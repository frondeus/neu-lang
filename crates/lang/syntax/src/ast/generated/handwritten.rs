#![allow(dead_code)]
use super::*;
use microtree::nodes;

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
impl Nodes {
    pub const Number: Name = Name::new("number");
    pub const Identifier: Name = Name::new("identifier");
    pub const ArticleItemId: Name = Name::new("item_id");
}
nodes! {
    Nodes,
    Markdown {
        MdParagraph,
        MdEmphasis,
        MdStrong,
        MdSoftBreak,
        MdHardBreak,
        MdRule,
        MdBlockQuote,
        MdUnorderedList,
        MdListItem,
        MdOrderedList,

        MdInlineLink,
        MdReferenceLink,
        MdShortcutLink,
        MdAutoLink,
        MdEmailLink,

        MdImage,
        MdInlineImage,
        MdReferenceImage,
        MdShortcutImage,
        MdAutoImage,
        MdEmailImage,

        MdLinkUrl,
        MdLinkTitle,

        MdImageSrc,
        MdImageTitle,

        MdReference,
        MdReferenceLabel,

        MdCodeBlock,
        MdCodeBlockLang,

        MdH1,
        MdH2,
        MdH3,
        MdH4,
        MdH5,
        MdH6,

        MdText,
        MdHtml
}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identifier(Red);
impl Ast for Identifier {
    fn new(node: Red) -> Option<Self> {
        Some(Self(node))
    }

    fn red(&self) -> Red {
        self.0.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArticleItemId(Red);
impl Ast for ArticleItemId {
    fn new(node: Red) -> Option<Self> {
        Some(Self(node))
    }

    fn red(&self) -> Red {
        self.0.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MdLinkUrl(Red);
impl Ast for MdLinkUrl {
    fn new(node: Red) -> Option<Self> {
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MdLinkTitle(Red);
impl Ast for MdLinkTitle {
    fn new(node: Red) -> Option<Self> {
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
