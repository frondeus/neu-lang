use crate::{MdStringLexer, MdStrToken};
use crate::Nodes;
use crate::core::*;
use neu_markdown::{transform_md, Event, Tag};

fn parse_event(builder: &mut NodeBuilder<MdStringLexer>,
               events: &mut impl PeekableIterator<Item = (Event, TextRange)>) {
    let (event, span) = match events.next() { Some(s) => s, None => return };
    match event {
        Event::Start(tag) => {
            builder.parse(node_mut(move |builder| {
                builder.name(match tag {
                    Tag::Emphasis => Nodes::MdEmphasis,
                    _ => Nodes::MdParagraph
                });
                builder.parse(node(|builder| {
                    builder.name(Nodes::MdBegin);
                    builder.set_span(span);
                }));
                while let Some((peeked, _)) = events.peek() {
                    if let Event::End(_) = peeked { break; }
                    parse_event(builder, events);
                }
                if let Some((_ ,end)) = events.next() {
                    builder.parse(node(move |builder| {
                        builder.name(Nodes::MdEnd);
                        builder.set_span(end);
                    }));
                }
            }))
        },
        Event::End(tag) => {},
        Event::Text => {
            builder.parse(node(move |builder| {
                builder.name(Nodes::MdText);
                builder.set_span(span);
            }));
        },
        Event::Code => {},
        Event::Html => {},
        Event::FootnoteReference => {},
        Event::SoftBreak => {},
        Event::HardBreak => {},
        Event::Rule => {},
        Event::TaskListMarker(bool) => {},
        _ => {}
    }
}

pub fn inner_md_string() -> impl Parser<MdStringLexer> {
    node(|builder: &mut NodeBuilder<MdStringLexer>| {
        builder.name(Nodes::Virtual);
        builder.name(Nodes::StrValue);
        let i = builder.state().lexer().state().input().clone();
        builder.parse(|state: &mut State<MdStringLexer>, ctx: &Context<MdStringLexer>| {
            let next = state.lexer_mut().next();
            let mut builder = NodeBuilder::new(state, ctx);
            if let Some(MdStrToken::Text) = next.as_kind() {
                let next = next.unwrap();
                let span = next.span;
                let str = i.range_span(span);
                let md_parser = pulldown_cmark::Parser::new(str);
                let mut md_parser = transform_md(md_parser, span.start());
                parse_event(&mut builder, &mut md_parser.peekable());
            }

            builder.build()
        });
    })
}
