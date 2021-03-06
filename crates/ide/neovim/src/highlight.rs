use neu_parser::Node;
use neu_syntax::Nodes;

pub trait NodeHighlight {
    fn highlight(&self) -> Option<&'static str>;
}

impl NodeHighlight for Node {
    fn highlight(&self) -> Option<&'static str> {
        if self.is(Nodes::Error) {
            Some("Error")
        } else if self.is(Nodes::Number) {
            Some("Float")
        } else if self.is(Nodes::Boolean) {
            Some("Boolean")
        } else if self.is(Nodes::Op) {
            Some("Operator")
        } else if self.is(Nodes::ArticleBody) {
            Some("None")
        } else if self.is_any(&[
            Nodes::Md_Emphasis,
            Nodes::Md_Strong,
            Nodes::Md_H1,
            Nodes::Md_H2,
            Nodes::Md_H3,
        ]) {
            Some("Operator")
        } else if self.is_any(&[Nodes::String, Nodes::Markdown]) && !self.is(Nodes::Md_Value) {
            Some("String")
        } else if self.is(Nodes::Md_Link) {
            Some("Underlined")
        } else if self.is(Nodes::Identifier) {
            Some("Identifier")
        } else if self.is(Nodes::Trivia) {
            Some("Comment")
        } else if self.is(Nodes::Interpolated) {
            Some("None")
        } else {
            None
        }
    }
}
