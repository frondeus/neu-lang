use crate::{Arena, CoreNodes, Name, Node, NodeId};

pub struct Children<'a, I> {
    iter: I,
    arena: &'a Arena,
}

impl<'a, I> Iterator for Children<'a, I>
where
    I: Iterator<Item = NodeId>,
{
    type Item = (NodeId, &'a Node);

    fn next(&mut self) -> Option<Self::Item> {
        let mut id = self.iter.next()?;
        let mut node = self.arena.get(id);
        while node.is(CoreNodes::Trivia) {
            id = self.iter.next()?;
            node = self.arena.get(id);
        }

        Some((id, node))
    }
}

impl<'a, I> Children<'a, I>
where
    I: Iterator<Item = NodeId>,
{
    pub fn new(iter: I, arena: &'a Arena) -> Self {
        Self { iter, arena }
    }

    pub fn find_node(&mut self, expected: Name) -> Option<(NodeId, &'a Node)> {
        let mut next = self.next()?;
        while !next.1.is(expected) {
            next = self.next()?;
        }
        Some(next)
    }
}
