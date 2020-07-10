use neu_parser::{NodeId, Arena, Children};
use crate::Nodes;

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

        let identifier = children.find_node(Nodes::Identifier)
            .map(|(identifier, _)| identifier);

        let item_id = children.find_node(Nodes::ArticleItemId)
            .map(|(item_id, _)| item_id);

        let strukt = children.find_node(Nodes::Struct)
            .map(|(strukt, _)| strukt);

        let body = children.find_node(Nodes::ArticleBody)
            .map(|(body, _)| body);

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
