use std::fmt::{Debug, Display};
use std::sync::Arc;

use smol_str::SmolStr;

#[derive(PartialEq, Eq, Clone, Copy, Default, PartialOrd, Ord, Hash)]
pub struct Name(&'static str);

impl Name {
    pub const fn new(s: &'static str) -> Self {
        Self(s)
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn value(&self) -> &'static str {
        self.0
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}
impl Debug for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct GreenData {
    pub name: Name,
    pub size: usize,
    pub kind: GreenKind,
}

#[derive(PartialEq, Eq, Clone, Hash)]
pub struct Green(pub(crate) Arc<GreenData>);

impl Green {
    pub fn name(&self) -> Name {
        self.0.name
    }

    pub fn is(&self, name: Name) -> bool {
        if let GreenKind::Alias(Some(green)) = &self.0.kind {
            if green.is(name) {
                return true;
            }
        }
        self.0.name == name
    }

    pub fn size(&self) -> usize {
        self.0.size
    }

    pub fn kind(&self) -> &GreenKind {
        &self.0.kind
    }

    pub fn is_alias(&self) -> bool {
        matches!(&self.0.kind, GreenKind::Alias(_))
    }

    pub fn as_node(&self) -> Option<&Node> {
        match &self.0.kind {
            GreenKind::Node(node) => Some(node),
            GreenKind::Alias(Some(g)) => g.as_node(),
            _ => None,
        }
    }

    pub fn as_token(&self) -> Option<&Token> {
        match &self.0.kind {
            GreenKind::Token(token) => Some(token),
            GreenKind::Alias(Some(g)) => g.as_token(),
            _ => None,
        }
    }

    pub fn children(&self) -> Box<dyn Iterator<Item = Green> + '_> {
        match &self.0.kind {
            GreenKind::Node(Node { children }) => Box::new(children.iter().cloned()),
            GreenKind::Alias(Some(green)) => green.children(),
            GreenKind::Alias(None) | GreenKind::Token(_) => Box::new(std::iter::empty()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Node {
    pub children: Vec<Green>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Token {
    pub leading: SmolStr,
    pub value: SmolStr,
    pub trailing: SmolStr,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum GreenKind {
    Node(Node),
    Alias(Option<Green>),
    Token(Token),
}

fn fmt_green(
    green: &Green,
    f: &mut std::fmt::Formatter<'_>,
    mut offset: usize,
    width: usize,
    skip_indent: bool,
) -> std::fmt::Result {
    if width > 0 && !skip_indent {
        write!(f, "{:width$}", " ", width = width)?;
    }
    write!(f, "{}", green.0.name)?;

    if let GreenKind::Alias(_) = &green.0.kind {
        write!(f, ", ")?;
    } else {
        write!(f, " @ {}..{}", offset, offset + green.size())?;
    }

    match &green.0.kind {
        GreenKind::Node(Node { children }) => {
            writeln!(f)?;
            let width = width + 4;
            for child in children {
                fmt_green(child, f, offset, width, false)?;
                offset += child.size();
            }
        }
        GreenKind::Alias(Some(child)) => {
            fmt_green(child, f, offset, width, true)?;
        }
        GreenKind::Alias(None) => {
            write!(f, " missing")?;
        }
        GreenKind::Token(Token {
            value,
            leading,
            trailing,
        }) => {
            write!(f, " = `{}`", fmt_debug_str(value))?;
            if !leading.is_empty() {
                write!(f, " ; leading: `{}`", fmt_debug_str(leading))?;
            }
            if !trailing.is_empty() {
                write!(f, " ; trailing: `{}`", fmt_debug_str(trailing))?;
            }
            writeln!(f)?;
        }
    }
    Ok(())
}

fn fmt_debug_str(s: &SmolStr) -> String {
    s.replace("\t", "\\t")
     .replace("\n", "\\n")
     .replace("`", "\\`")
}

impl Debug for Green {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let width = f.width().unwrap_or_default();
        writeln!(f, "--- GREEN TREE ---")?;
        fmt_green(self, f, 0, width, false)?;
        writeln!(f, "--- END ---")
    }
}

impl Display for Green {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0.kind {
            GreenKind::Node(Node { children }) => {
                children.iter().map(|c| write!(f, "{}", c)).collect()
            }
            GreenKind::Alias(Some(child)) => write!(f, "{}", child),
            GreenKind::Alias(None) => write!(f, ""),
            GreenKind::Token(Token {
                value,
                leading,
                trailing,
            }) => {
                write!(f, "{}", leading)?;
                write!(f, "{}", value)?;
                write!(f, "{}", trailing)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    struct Nodes;
    nodes! {
        Nodes,
        Test {
            Root,
            Add,
            number,
            op
        }
    }

    #[test]
    fn print() {
        let mut builder = Cache::default();
        let tree = builder.with_node(Nodes::Root, |builder| {
            vec![
                builder.token(Nodes::number, "2"),
                builder.token(Nodes::op, "+"),
                builder.token(Nodes::number, "2"),
            ]
        });

        let result = tree.to_string();

        assert_eq!("2+2", result);
    }

    #[test]
    fn nested() {
        let mut builder = Cache::default();
        let tree = builder.with_node(Nodes::Root, |builder| {
            vec![builder.with_node(Nodes::Add, |builder| {
                vec![
                    builder.token(Nodes::number, "2"),
                    builder.token(Nodes::op, "+"),
                    builder.token(Nodes::number, "2"),
                ]
            })]
        });

        let result = tree.to_string();

        assert_eq!("2+2", result);
    }

    #[test]
    fn builder() {
        let mut builder = Cache::default();
        builder.with_node(Nodes::Root, |builder| {
            vec![
                builder.token(Nodes::number, "2"),
                builder.token(Nodes::op, "+"),
                builder.token(Nodes::number, "2"),
            ]
        });

        assert_eq!(3, builder.size());
    }

    #[test]
    fn nameless() {
        let mut builder = Cache::default();
        let tree = builder.with_node(Nodes::Root, |builder| {
            vec![
                builder.with_node(Name::default(), |builder| {
                    vec! [
                        builder.token(Nodes::number, "2"),
                        builder.token(Nodes::op, "+"),
                        builder.token(Nodes::number, "2"),
                    ]
                })
            ]
        });

        dbg!(&tree);
    }

    #[test]
    fn print_trivia() {
        let mut builder = Cache::default();
        let tree = builder.with_node(Nodes::Root, |builder| {
            vec![
                builder.with_trivia(Nodes::number, "", "2", " "),
                builder.with_trivia(Nodes::op, "", "+", " "),
                builder.token(Nodes::number, "2"),
            ]
        });

        let result = tree.to_string();

        assert_eq!("2 + 2", result);
    }
}
