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
    pub fn build(id: NodeId, nodes: &Arena) -> Self {
        let node = nodes.get(id);

        let mut children = Children::new(node.children.iter().copied(), nodes)
            .find_node(Nodes::ArticleItem)
            .map(|(_, article)| {
                Children::new(article.children.iter().copied(), nodes)
            });

        let identifier = children.as_mut().and_then(|children| {
            let (identifier, _) = children.find_node(Nodes::Identifier)?;
            Some(identifier)
        });

        let item_id = children.as_mut().and_then(|children| {
            let (item_id, _) = children.find_node(Nodes::ArticleItemId)?;
            Some(item_id)
        });

        let strukt = children.as_mut().and_then(|children| {
            let (strukt, _) = children.find_node(Nodes::Struct)?;
            Some(strukt)
        });

        let body = children.as_mut().and_then(|children| {
            let (body, _) = children.find_node(Nodes::ArticleBody)?;
            Some(body)
        });

        Self {
            identifier,
            item_id,
            strukt,
            body
        }
    }

    pub fn anchor_body(self, nodes: &mut Arena) -> Self {
        match self.body {
            Some(body) => {
                let mut body = nodes.get_mut(body);
                body.parent = self.strukt;

                self
            },
            _ => self
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
