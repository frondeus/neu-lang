use crate::{Ancestors, Node, NodeId};
use std::borrow::Borrow;
use std::collections::BTreeMap;
use std::fmt;

#[derive(PartialEq, Eq, Clone)]
pub struct Arena<Node, Err> {
    nodes: Vec<Node>,
    errors: BTreeMap<NodeId, Err>,
}

impl<N, E> Default for Arena<N, E> {
    fn default() -> Self {
        Self {
            nodes: Default::default(),
            errors: Default::default(),
        }
    }
}

impl<Node: fmt::Debug, Err> fmt::Debug for Arena<Node, Err> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Arena")?;
        for (i, n) in self.nodes.iter().enumerate() {
            write!(f, "\tN{}: ", i)?;
            writeln!(f, "{:?}", n)?;
        }
        Ok(())
    }
}

impl<N: Node + Clone, E: Clone> Arena<N, E> {
    pub fn merge_errors(&mut self, other: &Self) {
        self.errors.extend(other.errors.clone().into_iter());
    }
}

impl<N: Node, E> Arena<N, E> {
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

    pub fn add_err(&mut self, id: NodeId, c: E) {
        self.errors.insert(id, c);
    }

    pub fn component(&self, id: NodeId) -> Option<&E> {
        self.errors.get(&id)
    }

    pub fn components(&self) -> impl Iterator<Item = (NodeId, &E)> {
        self.errors.iter().map(|(id, e)| (*id, e))
    }

    pub fn ancestors(&self, id: NodeId) -> Ancestors<N, E> {
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
