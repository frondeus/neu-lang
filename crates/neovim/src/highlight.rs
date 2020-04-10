use neu_parser::core::Node;
use neu_parser::Nodes;

pub trait NodeHighlight {
    fn highlight(&self) -> Option<&'static str>;
}

impl NodeHighlight for Node {
    fn highlight(&self) -> Option<&'static str> {
        if self.is(Nodes::Error) { Some("Error") }
        else if self.is(Nodes::Atom) { Some("Identifier") }
        else if self.is(Nodes::Token) { Some("Operator") }
        else { None }
    }
}

