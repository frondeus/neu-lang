use neu_parser::core::NodeId;

#[derive(Clone, Copy, Default, Debug)]
pub struct Context {
    top: Option<NodeId>,
    current: Option<NodeId>
}

impl Context {
    pub fn top(&self) -> Option<NodeId> {
        self.top
    }

    pub fn current(&self) -> Option<NodeId> {
        self.current
    }

    pub fn set_current(self, id: NodeId) -> Self {
        let mut top = self.top;

        if top.is_none() {
            top = Some(id);
        }

        Self {
            top, current: Some(id)
        }
    }
}
