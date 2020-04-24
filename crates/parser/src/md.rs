use crate::{MdStringLexer, MdStrToken, MainLexer};
use crate::Nodes;
use crate::core::*;
use pulldown_cmark::{Event, Tag, CowStr};
use text_size::{TextSize, TextRange, TextSized};
use std::convert::TryFrom;

fn offset(a: &str, orig: &str) -> usize {
    let a = a.as_ptr() as usize;
    let orig = orig.as_ptr() as usize;
    a - orig
}

fn get_range<'a>(str: &'a str, cow: CowStr<'a>, range: TextRange, from: TextSize) -> TextRange {
    match cow {
        CowStr::Boxed(_) => unreachable!("It should be never owned"),
        CowStr::Borrowed(s) => {
            let offset = TextSize::try_from(offset(s, str)).unwrap();
            TextRange(offset + from, offset + s.text_size() + from)
        },
        _ => range
    }
}

fn parse_start<'a>(
               span: TextRange,
               name: Name,
               str: &'a str,
               builder: &mut NodeBuilder<MdStringLexer>,
               events: &mut impl PeekableIterator<Item = (Event<'a>, TextRange)>,
               from: TextSize
) {
    builder.name(name);
    builder.set_span(span);
    while let Some((peeked, _)) = events.peek() {
        if let Event::End(_) = peeked {
            events.next();
            break;
        }
        parse_event(str, builder, events, from);
    }
}

fn parse_event<'a>(
    str: &'a str,
    builder: &mut NodeBuilder<MdStringLexer>,
    events: &mut impl PeekableIterator<Item = (Event<'a>, TextRange)>,
            from: TextSize
) {

    let (event, span) = match events.next() { Some(s) => s, None => return };
    match event {
        Event::Start(tag) => {
            let name = match tag {
                Tag::Emphasis => Nodes::Md_Emphasis,
                Tag::Paragraph => Nodes::Md_Paragraph,
                Tag::Heading(1) => Nodes::Md_H1,
                Tag::Heading(2) => Nodes::Md_H2,
                Tag::Heading(3) => Nodes::Md_H3,
                Tag::Heading(4) => Nodes::Md_H4,
                Tag::Heading(5) => Nodes::Md_H5,
                Tag::Heading(_) => Nodes::Md_H6,
                Tag::BlockQuote => todo!("BlockQuote"),
                Tag::CodeBlock(_) => todo!("CodeBlock"),
                Tag::List(_) => todo!("List"),
                Tag::Item => todo!("Item"),
                Tag::FootnoteDefinition(_) => todo!("FootnoteDefinition"),
                Tag::Table(_) => todo!("Table"),
                Tag::TableHead => todo!("TableHead"),
                Tag::TableRow => todo!("TableRow"),
                Tag::TableCell => todo!("TableCell"),
                Tag::Strong => Nodes::Md_Strong,
                Tag::Strikethrough => todo!("Strikethrough"),
                Tag::Link(_, _, _) => todo!("Link"),
                Tag::Image(_, _, _) => todo!("Image"),
            };
            if builder.span() == span {
                parse_start(span, name, str, builder, events, from);
            }
            else {
                builder.parse(node_mut(move |builder| {
                    parse_start(span, name, str, builder, events, from);
                }))
            }
        },
        Event::Text(cow) => {
            let range = get_range(str, cow, span, from);
            builder.parse(node(move |builder| {
                builder.name(Nodes::Md_Text);
                builder.set_span(range);
            }));
        },
        Event::Code(cow) => {
            let range = get_range(str, cow, span, from);
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
        Event::SoftBreak => {
            builder.parse(node(|builder| {
                builder.name(Nodes::Md_SoftBreak);
                builder.set_span(span);
            }));
        },
        Event::HardBreak => todo!("HardBreak"),
        Event::Rule => todo!("Rule"),
        Event::TaskListMarker(_bool) => todo!("TaskListMarker"),
        _ => {}
    }
}

pub fn inner_md_string() -> impl Parser<MdStringLexer> {
    |state: &mut State<MdStringLexer>, ctx: &Context<MdStringLexer>| {
        let i = state.lexer().state().input().clone();
        let next = state.lexer_mut().peek().as_kind();
        let mut builder = NodeBuilder::new(state, ctx);
        builder.name(Nodes::Virtual);
        builder.name(Nodes::Md_Value);
        if let Some(MdStrToken::Text) = next {
            let next = builder.state_mut().lexer_mut().next().unwrap();
            let span = next.span;
            let str = i.range_span(span);
            let from = span.start();
            let md_parser = pulldown_cmark::Parser::new(str)
                .into_offset_iter().map(|(event, range)| {
                let range = TextRange(
                    TextSize::try_from(range.start).unwrap() + from,
                    TextSize::try_from(range.end).unwrap() + from
                );
                (event, range)
            });
            let mut events = md_parser.peekable();
            while events.peek().is_some() {
                parse_event(str, &mut builder, &mut events,  from);
            }
        }

        builder.build()
    }
}
