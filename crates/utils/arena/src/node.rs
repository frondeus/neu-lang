use std::fmt;

pub trait Node {
    fn parent(&self) -> Option<NodeId>;
    fn set_parent(&mut self, id: NodeId);
    fn children(&self) -> &[NodeId];
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NodeId(pub(crate) usize);

impl fmt::Debug for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "N{}", self.0)
    }
}
