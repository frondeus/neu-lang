use neu_parser::core::Node;
use neu_parser::Nodes;

pub trait NodeHighlight {
    fn highlight(&self) -> Option<&'static str>;
}

impl NodeHighlight for Node {
    fn highlight(&self) -> Option<&'static str> {
        if self.is(Nodes::Error) { Some("Error") }
        else if self.is(Nodes::Number) { Some("Float") }
        else if self.is(Nodes::Boolean) { Some("Boolean") }
        else if self.is(Nodes::Op) { Some("Operator") }
        else if self.is(Nodes::String) { Some("String") }
        else { None }
    }
}

