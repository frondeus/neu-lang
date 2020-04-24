use crate::Eval;
use neu_parser::core::{Node, Name};
use neu_parser::Nodes;
use crate::context::Context;
use crate::children::Children;

impl<'a> Eval<'a> {
    pub(crate) fn eval_md(&mut self, str: &mut String, node: &Node, ctx: Context) -> Option<()> {
        const TAGS: &[(Name, &str)] = &[
            (Nodes::Md_Paragraph, "p"),
            (Nodes::Md_H1, "h1"),
            (Nodes::Md_H2, "h2"),
            (Nodes::Md_H3, "h3"),
            (Nodes::Md_H4, "h4"),
            (Nodes::Md_H5, "h5"),
            (Nodes::Md_H6, "h6"),
            (Nodes::Md_Emphasis, "em"),
            (Nodes::Md_Strong, "strong")
        ];

        let mut end_vec = vec![];
        for (tag_name, tag_str) in TAGS {
            if node.is(*tag_name) {
                str.push_str(&format!("<{}>", tag_str));
                end_vec.push(format!("</{}>", tag_str));
            }
        }

        if node.is(Nodes::Md_SoftBreak) {
            str.push_str("\n");
        }

        if node.is(Nodes::Md_Text) || node.is(Nodes::Md_Html) {
            let text = &self.input[node.span];
            str.push_str(text);
        }

        if node.is(Nodes::Interpolated) {
            let mut children = Children::new(node.children.iter().copied(), self.nodes);
            let (value_id, _) = children.find_node(Nodes::Value)?;
            let value = self.eager_eval(value_id, ctx)?;
            str.push_str(&value.to_string());
        }

        for id in node.children.iter() {
            let child = self.nodes.get(id);
            self.eval_md(str, child, ctx)?;
        }

        for end in end_vec.into_iter().rev() { str.push_str(&end); }

        Some(())
    }
}