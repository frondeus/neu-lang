use crate::{Components, Node, NodeId, Component, Ancestors};
use std::fmt;
use std::borrow::Borrow;

pub struct Arena<Node> {
    nodes: Vec<Node>,
    components: Components
}

impl<N> Default for Arena<N> {
    fn default() -> Self {
        Self {
            nodes: Default::default(),
            components: Default::default(),
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
    pub fn take(&mut self) -> Self {
        let nodes = std::mem::take(&mut self.nodes);
        let components = std::mem::take(&mut self.components);
        Self {
            nodes,
            components
        }
    }

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

    pub fn add_component(&mut self, id: NodeId, c: impl Component) {
        self.components.insert(id, c);
    }

    pub fn component<C: Component>(&self, id: NodeId) -> Option<&C> {
        self.components.get(id)
    }

    pub fn components<C: Component>(&self) -> impl Iterator<Item = (NodeId, &C)> {
        self.components.iter()
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

