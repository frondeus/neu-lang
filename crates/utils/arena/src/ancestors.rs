use crate::{NodeId, Arena, Node};

pub struct Ancestors<'a, Node> {
    pub(crate) current: Option<NodeId>,
    pub(crate) arena: &'a Arena<Node>,
}

impl<'a, N: Node> Iterator for Ancestors<'a, N> {
    type Item = NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current.map(|id| self.arena.get(id))?;

        let ancestor = current.parent();

        let current = self.current.take();
        self.current = ancestor;
        current
    }
}

