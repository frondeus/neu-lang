use crate::{MdStringLexer, MdStrToken, Token, MainLexer};
use crate::Nodes;
use crate::core::*;
use neu_markdown::{transform_md, Event, Tag};

fn parse_event(builder: &mut NodeBuilder<MdStringLexer>,
               events: &mut impl PeekableIterator<Item = (Event, TextRange)>
) {

    let (event, span) = match events.next() { Some(s) => s, None => return };
    match event {
        Event::Start(tag) => {
            let name = match tag {
                Tag::Emphasis => Nodes::MdEmphasis,
                _ => Nodes::MdParagraph
            };
            if builder.span() == span {
                builder.name(name);
                builder.set_span(span);
                while let Some((peeked, _)) = events.peek() {
                    if let Event::End(_) = peeked {
                        events.next();
                        break;
                    }
                    parse_event(builder, events);
                }
            }
            else {
                builder.parse(node_mut(move |builder| {
                    builder.name(name);
                    builder.set_span(span);
                    while let Some((peeked, _)) = events.peek() {
                        if let Event::End(_) = peeked {
                            events.next();
                            break;
                        }
                        parse_event(builder, events);
                    }
                }))
            }
        },
        Event::Text(span) => {
            builder.parse(node(move |builder| {
                builder.name(Nodes::MdText);
                builder.set_span(span.as_borrowed().expect("origin in text"));
            }));
        },
        Event::Code(span) => {
            let range = span.as_borrowed().expect("origin in text");
            let ctx = Context::default();
            builder.parse_mode(&ctx, node(move |builder: &mut NodeBuilder<MainLexer>| {
                let saved = builder.state_mut().lexer_mut().state_mut().input().clone();

                builder.state_mut().lexer_mut().state_mut().input_mut().set_range(range);
                builder.name(Nodes::Virtual);
                builder.name(Nodes::Interpolated);
                builder.parse(crate::neu::parser());

                *builder.state_mut().lexer_mut().state_mut().input_mut() = saved;
            }));
            //let lexer = MainLexer::build(input.into());
        },
        Event::Html(_span) => todo!("Html"),
        Event::FootnoteReference(_span) => todo!("FootnoteReference"),
        Event::SoftBreak => todo!("SoftBreak"),
        Event::HardBreak => todo!("HardBreak"),
        Event::Rule => todo!("Rule"),
        Event::TaskListMarker(_bool) => todo!("TaskListMarker"),
        _ => {}
    }
}

pub fn inner_md_string() -> impl Parser<MdStringLexer> {
    |state: &mut State<MdStringLexer>, ctx: &Context<MdStringLexer>| {
        let i = state.lexer().state().input().clone();
        let next = state.lexer_mut().next();
        let mut builder = NodeBuilder::new(state, ctx);
        builder.name(Nodes::Virtual);
        if let Some(MdStrToken::Text) = next.as_kind() {
            let next = next.unwrap();
            let span = next.span;
            let str = i.range_span(span);
            let md_parser = pulldown_cmark::Parser::new(str);
            let md_parser = transform_md(md_parser, span.start(), str);
            parse_event(&mut builder, &mut md_parser.peekable());
        }

        builder.build()
    }
}
