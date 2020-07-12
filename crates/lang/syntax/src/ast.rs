use crate::Nodes;
use neu_parser::{Arena, Children, Node, NodeId, ParseResult};

pub trait RootAst: Default + Ast {
    //fn from_root_syntax(id: NodeId, nodes: &Arena) -> Self where Self: Sized {
    fn from_root_syntax(ParseResult { root, arena, .. }: &ParseResult) -> Self
    where
        Self: Sized,
    {
        arena
            .get(root)
            .children
            .iter()
            .filter_map(|id| Self::from_syntax(*id, arena))
            .next()
            .unwrap_or_default()
    }
}

impl<A> RootAst for A where A: Default + Ast {}

pub trait Ast {
    fn from_syntax(id: NodeId, nodes: &Arena) -> Option<Self>
    where
        Self: Sized;
}

#[derive(Debug)]
pub struct ArticleRef {
    pub identifier: Option<NodeId>,
    pub item_id: Option<NodeId>,
}

impl Ast for ArticleRef {
    fn from_syntax(id: NodeId, nodes: &Arena) -> Option<Self> {
        let node = nodes.get(id);
        if !node.is(Nodes::ArticleRef) {
            return None;
        }

        let mut children = Children::new(node.children.iter().copied(), nodes);

        let identifier = children.find_node(Nodes::Identifier).map(get_id);

        let item_id = children.find_node(Nodes::ArticleItemId).map(get_id);

        Some(Self {
            identifier,
            item_id,
        })
    }
}

impl ArticleRef {
    pub fn identifier<'a>(&self, nodes: &Arena, input: &'a str) -> Option<&'a str> {
        let identifier = self.identifier?;
        let node = nodes.get(identifier);
        Some(&input[node.span])
    }

    pub fn item_id<'a>(&self, nodes: &Arena, input: &'a str) -> Option<&'a str> {
        let item_id = self.item_id?;
        let node = nodes.get(item_id);
        Some(&input[node.span])
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ArticleItem {
    pub identifier: Option<NodeId>,
    pub item_id: Option<NodeId>,
    pub strukt: Option<NodeId>,
    pub body: Option<NodeId>,
}

impl Ast for ArticleItem {
    fn from_syntax(id: NodeId, nodes: &Arena) -> Option<Self> {
        let node = nodes.get(id);
        if !node.is(Nodes::ArticleItem) {
            return None;
        }

        let mut children = Children::new(node.children.iter().copied(), nodes);

        let identifier = children.find_node(Nodes::Identifier).map(get_id);
        let item_id = children.find_node(Nodes::ArticleItemId).map(get_id);
        let strukt = children.find_node(Nodes::Struct).map(get_id);
        let body = children.find_node(Nodes::ArticleBody).map(get_id);

        Some(Self {
            identifier,
            item_id,
            strukt,
            body,
        })
    }
}

impl ArticleItem {
    pub fn anchor_body(&self, nodes: &mut Arena) {
        if let Some(body) = self.body {
            let mut body = nodes.get_mut(body);
            body.parent = self.strukt;
        }
    }

    pub fn identifier<'a>(&self, nodes: &Arena, input: &'a str) -> Option<&'a str> {
        let identifier = self.identifier?;
        let node = nodes.get(identifier);
        Some(&input[node.span])
    }

    pub fn item_id<'a>(&self, nodes: &Arena, input: &'a str) -> Option<&'a str> {
        let item_id = self.item_id?;
        let node = nodes.get(item_id);
        Some(&input[node.span])
    }
}

fn get_id((item_id, _node): (NodeId, &Node)) -> NodeId {
    item_id
}
