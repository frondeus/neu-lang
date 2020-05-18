use crate::{Arena, NodeId, TextRange};
use itertools::Itertools;
use std::collections::BTreeSet;
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Name(pub(crate) &'static str);
impl Name {
    pub const fn new(name: &'static str) -> Self {
        Self(name)
    }
}

#[macro_export]
macro_rules! nodes {
    () => {
        pub struct Nodes;
        #[allow(non_upper_case_globals)]
        impl Nodes {
            pub const Virtual: Name = Name::new("Virtual");
            pub const Root: Name = Name::new("Root");
            pub const Trivia: Name = Name::new("Trivia");
            pub const Token: Name = Name::new("Token");
            pub const Error: Name = Name::new("Error");
        }
    };
    (
        $($group: ident {
            $($node: ident),*
        }),*
    ) => {
        nodes!();
        #[allow(non_upper_case_globals)]
        impl Nodes {
            $( $( pub const $node: Name = Name::new(stringify!($node)); )* )*
        }
    }
}

impl fmt::Debug for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    pub span: TextRange,
    pub names: BTreeSet<Name>,
    pub children: Vec<NodeId>,
    pub parent: Option<NodeId>,
}

impl Node {
    pub fn empty() -> Self {
        Self {
            names: Default::default(),
            children: Default::default(),
            parent: Default::default(),
            span: TextRange::empty(0.into()),
        }
    }

    pub fn is_any(&self, names: &[Name]) -> bool {
        names.iter().any(|name| self.is(*name))
    }

    pub fn is_all(&self, names: &[Name]) -> bool {
        names.iter().all(|name| self.is(*name))
    }

    pub fn is(&self, name: Name) -> bool {
        self.names.contains(&name)
    }

    pub fn with_name(mut self, name: Name) -> Node {
        self.names.insert(name);
        self
    }

    pub fn display<'s, 'n, 'a>(
        &'n self,
        str: &'s str,
        arena: &'a Arena,
    ) -> DisplayNode<'s, 'n, 'a> {
        DisplayNode {
            str,
            node: self,
            arena,
        }
    }

    pub fn parent(&self) -> Option<NodeId> {
        self.parent
    }
}

pub struct DisplayNode<'s, 'n, 'a> {
    str: &'s str,
    node: &'n Node,
    arena: &'a Arena,
}

impl<'s, 'n, 'a> fmt::Display for DisplayNode<'s, 'n, 'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width = f.width().unwrap_or_default();
        if width > 0 {
            write!(f, "{:width$}", " ", width = width)?;
        }

        let span = &self.str[self.node.span].escape_default();
        writeln!(
            f,
            "{} @ {:?} = `{}`",
            self.node.names.iter().join(", ").to_uppercase(),
            self.node.span,
            span
        )?;
        let c_width = width + 4;
        for child in self
            .node
            .children
            .iter()
            .map(|child| self.arena.get(child).display(self.str, self.arena))
        {
            write!(f, "{:width$}", child, width = c_width)?;
        }
        Ok(())
    }
}
