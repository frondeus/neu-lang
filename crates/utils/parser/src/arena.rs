use crate::Node;
pub use neu_arena::NodeId;
use neu_diagnostics::Diagnostic;

pub type Arena<N = Node, E = Diagnostic> = neu_arena::Arena<N, E>;
pub type Ancestors<'a, N = Node, E = Diagnostic> = neu_arena::Ancestors<'a, N, E>;

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
