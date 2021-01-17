#![allow(dead_code)]
use super::*;
use microtree::nodes;

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
impl Nodes {
    pub const Number: Name = Name::new("number");
    pub const Text: Name = Name::new("token");
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Number(Red);
impl Ast for Number {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::Number) {
            return None;
        }
        Some(Self(node))
    }

    fn red(&self) -> Red {
        self.0.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier(Red);
impl Ast for Identifier {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::Identifier) {
            return None;
        }
        Some(Self(node))
    }

    fn red(&self) -> Red {
        self.0.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Text(Red);
impl Ast for Text {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::Text) {
            return None;
        }
        Some(Self(node))
    }

    fn red(&self) -> Red {
        self.0.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ItemIdent(Red);
impl Ast for ItemIdent {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::Identifier) {
            return None;
        }
        Some(Self(node))
    }

    fn red(&self) -> Red {
        self.0.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArticleItemId(Red);
impl Ast for ArticleItemId {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::ArticleItemId) {
            return None;
        }
        Some(Self(node))
    }

    fn red(&self) -> Red {
        self.0.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MdLinkUrl(Red);
impl Ast for MdLinkUrl {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::MdLinkUrl) {
            return None;
        }
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MdLinkTitle(Red);
impl Ast for MdLinkTitle {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::MdLinkTitle) {
            return None;
        }
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MdCodeBlockLang(Red);
impl Ast for MdCodeBlockLang {
    fn new(node: Red) -> Option<Self> {
        if !node.is(Nodes::MdCodeBlockLang) {
            return None;
        }
        Some(Self(node))
    }
    fn red(&self) -> Red {
        self.0.clone()
    }
}
