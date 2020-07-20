use crate::Node;
pub use neu_arena::NodeId;
pub use neu_diagnostics::{Diagnostic, Diagnostics};

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

