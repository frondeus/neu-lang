use crate::{MdStringLexer, MdStrToken, MainLexer};
use crate::Nodes;
use crate::core::*;
use pulldown_cmark::{Event, Tag, CowStr, LinkType};
use text_size::{TextSize, TextRange, TextSized};
use std::convert::TryFrom;

fn offset(a: &str, orig: &str) -> Option<usize> {
    let a = a.as_ptr() as usize;
    let orig = orig.as_ptr() as usize;
    if a > orig {
        Some(a - orig)
    } else {
        None
    }
}

fn get_range<'a>(str: &'a str, cow: &CowStr<'a>, range: TextRange, from: TextSize) -> TextRange {
    match cow {
        CowStr::Boxed(_) => unreachable!("It should be never owned"),
        CowStr::Borrowed(s) => {
            let offset = offset(s, str).unwrap_or(0);
            let offset = TextSize::try_from(offset).unwrap();
            TextRange(offset + from, offset + s.text_size() + from)
        },
        _ => range
    }
}

fn parse_start<'a>(
               span: TextRange,
               tag: &Tag,
               str: &'a str,
               builder: &mut NodeBuilder<MdStringLexer>,
               events: &mut impl PeekableIterator<Item = (Event<'a>, TextRange)>,
               from: TextSize
) {
    builder.set_span(span);
    let name = match tag {
        Tag::Emphasis => Nodes::Md_Emphasis,
        Tag::Paragraph => Nodes::Md_Paragraph,
        Tag::Heading(1) => Nodes::Md_H1,
        Tag::Heading(2) => Nodes::Md_H2,
        Tag::Heading(3) => Nodes::Md_H3,
        Tag::Heading(4) => Nodes::Md_H4,
        Tag::Heading(5) => Nodes::Md_H5,
        Tag::Heading(_) => Nodes::Md_H6,
        Tag::BlockQuote => Nodes::Md_BlockQuote,
        Tag::CodeBlock(_) => todo!("CodeBlock"),
        Tag::List(None) => Nodes::Md_UnorderedList,
        Tag::List(Some(1)) => Nodes::Md_OrderedList,
        Tag::List(_offset) => {
            //TODO: OrderedList
            Nodes::Md_OrderedList
        },
        Tag::Item => Nodes::Md_ListItem,
        Tag::FootnoteDefinition(_) => todo!("FootnoteDefinition"),
        Tag::Table(_) => todo!("Table"),
        Tag::TableHead => todo!("TableHead"),
        Tag::TableRow => todo!("TableRow"),
        Tag::TableCell => todo!("TableCell"),
        Tag::Strong => Nodes::Md_Strong,
        Tag::Strikethrough => todo!("Strikethrough"),
        Tag::Link(link_type, url, title) => {
            let url_range = get_range(str, url, span, from);
            let title_range = get_range(str, title, span, from);
            builder.parse(node(|builder| {
                builder.name(Nodes::Md_LinkUrl);
                builder.set_span(url_range);
            }));
            builder.parse(node(|builder| {
                builder.name(Nodes::Md_LinkTitle);
                builder.set_span(title_range);
            }));
            builder.name(Nodes::Md_Link);
            match link_type {
                LinkType::Inline => Nodes::Md_InlineLink,
                LinkType::Reference => Nodes::Md_ReferenceLink,
                LinkType::ReferenceUnknown => todo!("LinkType LinkType::ReferenceUnknown"),
                LinkType::Collapsed => todo!("LinkType LinkType::Collapsed"),
                LinkType::CollapsedUnknown => todo!("LinkType LinkType::CollapsedUnknown"),
                LinkType::Shortcut => Nodes::Md_ShortcutLink,
                LinkType::ShortcutUnknown => todo!("LinkType LinkType::ShortcutUnknown"),
                LinkType::Autolink => Nodes::Md_AutoLink,
                LinkType::Email => Nodes::Md_EmailLink,
            }
        },
        Tag::Image(_, _, _) => todo!("Image"),
    };
    builder.name(name);
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
            if builder.span() == span {
                parse_start(span, &tag, str, builder, events, from);
            }
            else {
                builder.parse(node_mut(move |builder| {
                    parse_start(span, &tag, str, builder, events, from);
                }))
            }
        },
        Event::Text(cow) => {
            let range = get_range(str, &cow, span, from);
            builder.parse(node(move |builder| {
                builder.name(Nodes::Md_Text);
                builder.set_span(range);
            }));
        },
        Event::Code(cow) => {
            let range = get_range(str, &cow, span, from);
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
        Event::Rule => {
            builder.parse(node(|builder| {
                builder.name(Nodes::Md_Rule);
                builder.set_span(span);
            }));
        },
        Event::TaskListMarker(_bool) => todo!("TaskListMarker"),
        _ => {}
    }
}

pub fn inner_md_string(hash: usize) -> impl Parser<MdStringLexer> {
    move |state: &mut State<MdStringLexer>, ctx: &Context<MdStringLexer>| {
        state.lexer_mut().set_hash(hash);
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
