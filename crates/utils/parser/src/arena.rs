pub use neu_arena::{NodeId};
use crate::Node;
use neu_diagnostics::Diagnostic;

pub type Arena<N = Node> = neu_arena::Arena<N>;
pub type Ancestors<'a, N = Node> = neu_arena::Ancestors<'a, N>;

impl neu_arena::Node for Node {
    fn parent(&self) -> Option<NodeId> {
        self.parent
    }

    fn set_parent(&mut self, id: NodeId) {
        self.parent = Some(id);
    }

    fn children(&self) -> &[NodeId] {
        &self.children
    }
}

pub trait ArenaExt {
    fn errors(&self) -> Vec<(NodeId, &Diagnostic)>;
}

impl ArenaExt for Arena {
    fn errors(&self) -> Vec<(NodeId, &Diagnostic)> {
        let mut v = self.components().collect::<Vec<_>>();
        v.sort_by_key(|f| f.0);
        v
    }
}