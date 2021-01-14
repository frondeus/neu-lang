use std::sync::Arc;

use text_size::{TextRange, TextSize};

use crate::Green;
use crate::Name;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RedData {
    kind: RedKind,
    green: Green,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RedKind {
    Root,
    Child {
        parent: Red,
        index: usize,
        offset: usize,
    },
}

impl RedKind {
    pub fn as_child(&self) -> Option<(&Red, usize, usize)> {
        match self {
            Self::Child {
                parent,
                index,
                offset,
            } => Some((parent, *index, *offset)),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Eq, Hash)]
pub struct Red(Arc<RedData>);

impl std::fmt::Display for Red {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.green(), f)
    }
}

impl Red {
    pub fn root(green: Green) -> Self {
        Self(Arc::new(RedData {
            kind: RedKind::Root,
            green,
        }))
    }

    pub fn child(green: Green, parent: Red, index: usize, offset: usize) -> Self {
        Self(Arc::new(RedData {
            kind: RedKind::Child {
                parent,
                index,
                offset,
            },
            green,
        }))
    }

    pub fn green(&self) -> Green {
        self.0.green.clone()
    }

    pub fn is_alias(&self) -> bool {
        self.0.green.is_alias()
    }

    pub fn is_node(&self) -> bool {
        self.0.green.as_node().is_some()
    }

    pub fn kind(&self) -> &RedKind {
        &self.0.kind
    }

    pub fn name(&self) -> Name {
        self.green().name()
    }

    pub fn is(&self, name: Name) -> bool {
        self.green().is(name)
    }

    pub fn parent(&self) -> Option<Red> {
        let parent = self.0.kind.as_child()?.0;
        Some(parent.clone())
    }

    pub fn offset(&self) -> usize {
        self.0
            .kind
            .as_child()
            .map(|(_, _, o)| o)
            .unwrap_or_default()
    }

    pub fn range(&self) -> TextRange {
        let from = TextSize::from(self.offset() as u32);
        let len = self.0.green.size();
        let len = TextSize::from(len as u32);
        TextRange::at(from, len)
    }

    pub fn ancestors(&self) -> impl Iterator<Item = Red> + '_ {
        std::iter::successors(self.parent(), |parent| {
            parent.parent()
        })
    }

    pub fn children(&self) -> impl Iterator<Item = Red> + '_ {
        let parent = self.clone();
        let mut offset = self.offset();
        self.0
            .green
            .children()
            .enumerate()
            .map(move |(idx, green_child)| {
                let child_offset = offset;
                offset += green_child.size();
                Self::child(green_child, parent.clone(), idx, child_offset)
            })
    }

    pub fn pre_order(&self) -> impl Iterator<Item = Red> + '_ {
        Some(self.clone()).into_iter()
        .chain(
            self.children()
                .flat_map(|child| child.pre_order().collect::<Vec<_>>())
        )
    }

    pub fn post_order(&self) -> impl Iterator<Item = Red> + '_ {
        self.children()
            .flat_map(|child| child.post_order().collect::<Vec<_>>())
            .chain(Some(self.clone()).into_iter())
    }
}

impl PartialEq for Red {
    fn eq(&self, other: &Self) -> bool {
        self.offset() == other.offset() && self.0.green == other.0.green
    }
}
