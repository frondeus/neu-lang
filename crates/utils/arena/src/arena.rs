use crate::{Ancestors, Node, NodeId};
use std::borrow::Borrow;
use std::fmt;

#[derive(PartialEq, Eq, Clone)]
pub struct Arena<Node> {
    nodes: Vec<Node>,
}

impl<N> Default for Arena<N> {
    fn default() -> Self {
        Self {
            nodes: Default::default(),
        }
    }
}

impl<Node: fmt::Debug> fmt::Debug for Arena<Node> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Arena")?;
        for (i, n) in self.nodes.iter().enumerate() {
            write!(f, "\tN{}: ", i)?;
            writeln!(f, "{:?}", n)?;
        }
        Ok(())
    }
}

impl<N: Node> Arena<N> {
    pub fn add(&mut self, node: N) -> NodeId {
        let len = self.nodes.len();
        let id = NodeId(len);
        for child_id in node.children().iter() {
            let child = self.get_mut(child_id);
            child.set_parent(id);
        }
        self.nodes.push(node);
        id
    }

    pub fn ancestors(&self, id: NodeId) -> Ancestors<N> {
        Ancestors {
            current: Some(id),
            arena: self,
        }
    }

    pub fn get(&self, id: impl Borrow<NodeId>) -> &N {
        let id = *id.borrow();
        &self.nodes[id.0]
    }

    pub fn get_mut(&mut self, id: impl Borrow<NodeId>) -> &mut N {
        let id = *id.borrow();
        &mut self.nodes[id.0]
    }

    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &N> {
        self.nodes.iter()
    }

    pub fn enumerate(&self) -> impl Iterator<Item = (NodeId, &N)> {
        self.nodes.iter().enumerate().map(|(id, n)| (NodeId(id), n))
    }
}
