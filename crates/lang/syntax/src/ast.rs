use neu_parser::{NodeId, Arena, Children, Node};
use crate::Nodes;

#[derive(Debug)]
pub struct ArticleRef {
    pub identifier: Option<NodeId>,
    pub item_id: Option<NodeId>,
}

fn get_id((item_id, _node): (NodeId, &Node)) -> NodeId {
    item_id
}

impl ArticleRef {
    pub fn build(id: NodeId, nodes: &Arena) -> Self {
        let node = nodes.get(id);

        let mut children = Children::new(node.children.iter().copied(), nodes);

        let identifier = children.find_node(Nodes::Identifier).map(get_id);

        let item_id = children.find_node(Nodes::ArticleItemId).map(get_id);

        Self {
            identifier,
            item_id,
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

#[derive(Debug)]
pub struct ArticleItem {
    pub identifier: Option<NodeId>,
    pub item_id: Option<NodeId>,
    pub strukt: Option<NodeId>,
    pub body:   Option<NodeId>
}

impl ArticleItem {
    pub fn from_root(id: NodeId, nodes: &Arena) -> Self {
        let node = nodes.get(id);

        match Children::new(node.children.iter().copied(), nodes)
            .find_node(Nodes::ArticleItem) {
            Some((article_id, _)) => Self::build(article_id, nodes),
            None => {
                Self {
                    identifier: None,
                    item_id: None,
                    strukt: None,
                    body: None
                }
            }
        }
    }
    pub fn build(id: NodeId, nodes: &Arena) -> Self {
        let node = nodes.get(id);

        let mut children = Children::new(node.children.iter().copied(), nodes);

        let identifier = children.find_node(Nodes::Identifier).map(get_id);
        let item_id = children.find_node(Nodes::ArticleItemId).map(get_id);
        let strukt = children.find_node(Nodes::Struct).map(get_id);
        let body = children.find_node(Nodes::ArticleBody).map(get_id);

        Self {
            identifier,
            item_id,
            strukt,
            body
        }
    }

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
