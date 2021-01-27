use crate::Eval;
use neu_syntax::{Nodes, ast::Interpolated, reexport::{Ast, Name, Red}};
use regex::Regex;

impl Eval {
    pub(crate) fn eval_md(&mut self, str: &mut String, node: Red) -> Option<()> {
        const TAGS: &[(Name, &str)] = &[
            (Nodes::MdParagraph, "p"),
            (Nodes::MdH1, "h1"),
            (Nodes::MdH2, "h2"),
            (Nodes::MdH3, "h3"),
            (Nodes::MdH4, "h4"),
            (Nodes::MdH5, "h5"),
            (Nodes::MdH6, "h6"),
            (Nodes::MdEmphasis, "em"),
            (Nodes::MdStrong, "strong"),
            (Nodes::MdBlockQuote, "blockquote"),
            (Nodes::MdUnorderedList, "ul"),
            (Nodes::MdOrderedList, "ol"),
            (Nodes::MdListItem, "li"),
        ];

        let mut end_vec = vec![];
        for (tag_name, tag_str) in TAGS {
            if node.is(*tag_name) {
                str.push_str(&format!("<{}>", tag_str));
                end_vec.push(format!("</{}>", tag_str));
            }
        }

        if node.is(Nodes::MdCodeBlock) {
            str.push_str("<pre><code");

            if let Some(node) = node.children().find(|n| n.is(Nodes::MdCodeBlockLang)) {
                let text = Self::str_non_trivia(node);
                str.push_str(&format!(r#" class="language-{}""#, text));
            }
            str.push_str(">");
            end_vec.push("</code></pre>".into());
        }

        if node.is(Nodes::MdImage) {
            //dbg!(&node);
            str.push_str("<img");
            if let Some(node) = node.children().find(|n| n.is(Nodes::MdImageSrc)) {
                let text = Self::str_non_trivia(node);
                str.push_str(&format!(r#" src="{}""#, text));
            }
            if let Some(_node) = node.children().find(|n| n.is(Nodes::MdImageTitle)) {
                //let text = &self.input[node.span];
                //str.push_str(&format!(r#" title="{}""#, text));
                //TODO: Disabled title:
                // Because of Pulldown cmark using inline string which loses
                // information about the title span.
            }
            str.push_str(">");
            end_vec.push("</img>".into());
        }

        if node.is(Nodes::MdLink) {
            //dbg!(&node);
            str.push_str("<a");
            if let Some(node) = node.children().find(|n| n.is(Nodes::MdLinkUrl)) {
                let text = Self::str_non_trivia(node);
                //TODO Parse :
                let link_regex = Regex::new("([a-z_A-Z0-9]+):([0-9A-Fa-f]{8})").expect("Regex");
                if let Some(cap) = link_regex.captures(&text) {
                    let kind = cap.get(1).expect("G1").as_str();
                    let id = cap.get(2).expect("G2").as_str();
                    str.push_str(&format!(r#" href="/{}/{}""#, kind, id));
                } else {
                    str.push_str(&format!(r#" href="{}""#, text));
                }
            }
            if let Some(_node) = node.children().find(|n| n.is(Nodes::MdLinkTitle)) {
                //let text = &self.input[node.span];
                //str.push_str(&format!(r#" title="{}""#, text));
                //TODO: Disabled title:
                // Because of Pulldown cmark using inline string which loses
                // information about the title span.
            }
            str.push_str(">");
            end_vec.push("</a>".into());
        }



        if let Some(interpolated) = Interpolated::new(node.clone()) {
            let value = interpolated.value()?;
            let value = self.eager_eval(value.red(), true)?;
            str.push_str("<pre><code>");
            str.push_str(&format!("{:#}", value));
            end_vec.push("</code></pre>".into());
        }

        if node.is(Nodes::MdRule) {
            str.push_str("<hr/>")
        }

        if node.is(Nodes::MdSoftBreak) {
            str.push_str("\n");
        }

        if node.is(Nodes::MdText) || node.is(Nodes::MdHtml) {
            let text = Self::str_non_trivia(node.clone());
            str.push_str(&text);
        }

        for child in node.children() {
            self.eval_md(str, child)?;
        }
        for end in end_vec.into_iter().rev() {
            str.push_str(&end);
        }
        Some(())
    }
}

