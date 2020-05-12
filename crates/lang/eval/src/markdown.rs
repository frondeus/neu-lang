use crate::Eval;
use neu_parser::{Name, Node, Children};
use neu_syntax::Nodes;

impl<'a> Eval<'a> {
    pub(crate) fn eval_md(&mut self, str: &mut String, node: &Node) -> Option<()> {
        const TAGS: &[(Name, &str)] = &[
            (Nodes::Md_Paragraph, "p"),
            (Nodes::Md_H1, "h1"),
            (Nodes::Md_H2, "h2"),
            (Nodes::Md_H3, "h3"),
            (Nodes::Md_H4, "h4"),
            (Nodes::Md_H5, "h5"),
            (Nodes::Md_H6, "h6"),
            (Nodes::Md_Emphasis, "em"),
            (Nodes::Md_Strong, "strong"),
            (Nodes::Md_BlockQuote, "blockquote"),
            (Nodes::Md_UnorderedList, "ul"),
            (Nodes::Md_OrderedList, "ol"),
            (Nodes::Md_ListItem, "li"),
        ];

        let mut end_vec = vec![];
        for (tag_name, tag_str) in TAGS {
            if node.is(*tag_name) {
                str.push_str(&format!("<{}>", tag_str));
                end_vec.push(format!("</{}>", tag_str));
            }
        }

        if node.is(Nodes::Md_CodeBlock) {
            let mut children = Children::new(node.children.iter().copied(), self.nodes);
            str.push_str("<pre><code");

            if let Some((_, node)) = children.find_node(Nodes::Md_CodeBlockLang) {
                let text = &self.input[node.span];
                str.push_str(&format!(r#" class="language-{}""#, text));
            }
            str.push_str(">");
            end_vec.push("</code></pre>".into());
        }

        if node.is(Nodes::Md_Image) {
            let mut children = Children::new(node.children.iter().copied(), self.nodes);
            str.push_str("<img");
            if let Some((_, node)) = children.find_node(Nodes::Md_ImageSrc) {
                let text = &self.input[node.span];
                str.push_str(&format!(r#" src="{}""#, text));
            }
            if let Some((_, _node)) = children.find_node(Nodes::Md_ImageTitle) {
                //let text = &self.input[node.span];
                //str.push_str(&format!(r#" title="{}""#, text));
                //TODO: Disabled title:
                // Because of Pulldown cmark using inline string which loses
                // information about the title span.
            }
            str.push_str(">");
            end_vec.push("</img>".into());
        }

        if node.is(Nodes::Md_Link) {
            let mut children = Children::new(node.children.iter().copied(), self.nodes);
            str.push_str("<a");
            if let Some((_, node)) = children.find_node(Nodes::Md_LinkUrl) {
                let text = &self.input[node.span];
                str.push_str(&format!(r#" href="{}""#, text));
            }
            if let Some((_, _node)) = children.find_node(Nodes::Md_LinkTitle) {
                //let text = &self.input[node.span];
                //str.push_str(&format!(r#" title="{}""#, text));
                //TODO: Disabled title:
                // Because of Pulldown cmark using inline string which loses
                // information about the title span.
            }
            str.push_str(">");
            end_vec.push("</a>".into());
        }

        if node.is(Nodes::Md_Rule) {
            str.push_str("<hr/>")
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
            let value = self.eager_eval(value_id, true)?;
            str.push_str(&value.to_string());
        }

        for id in node.children.iter() {
            let child = self.nodes.get(id);
            self.eval_md(str, child)?;
        }

        for end in end_vec.into_iter().rev() {
            str.push_str(&end);
        }

        Some(())
    }
}
