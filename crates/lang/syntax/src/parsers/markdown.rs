use std::convert::TryFrom;

use crate::{lexers::md_string::Token as MdStrToken, lexers::neu::Token as NeuToken, HashCount};
use crate::Nodes;
use microtree::Name;
use microtree_parser::*;
use microtree_parser::parsers::*;
use pulldown_cmark::{CodeBlockKind, CowStr, Event, LinkType, Tag};
use text_size::{TextLen, TextRange, TextSize};

pub(crate) fn inner_md_string<'s>() -> impl Parser<'s, MdStrToken> {
    node(|builder| {
        let mut builder = builder.node();
        if let Some(MdStrToken::Text) = builder.peek_token() {
            builder = markdown(builder.name(Nodes::MdValue));
        }
        builder.finish()
    })
}

pub(crate) fn markdown<'s, 'c, Token>(mut builder: NodeBuilder<'s, 'c, Token>) -> NodeBuilder<'s, 'c, Token>
where
    Token: TokenKind<'s> + std::fmt::Debug,
    Token::Extras: From<HashCount> + Into<HashCount>,
{
    let state = builder.state_mut();
    let next = state.lexer_mut().next().unwrap();

    let from = next.range.start();
    let str = next.value;
    let str_len: TextSize  = (str.len() as u32).into();

    let md_parser = pulldown_cmark::Parser::new(&str)
        .into_offset_iter()
        .map(|(event, range)| {
            let range = TextRange::new(
                TextSize::try_from(range.start).unwrap() + from,
                TextSize::try_from(range.end).unwrap() + from,
            );
            (event, range)
        });
    let mut events = md_parser.peekable();
    while events.peek().is_some() {
        builder = parse_event(builder,
                              TextRange::up_to(str_len),
                              &str, &mut events, from);
    }

    builder
}

fn parse_event<'s, 'c, 'a, Token, Events>(
    builder: NodeBuilder<'s, 'c, Token>,
    parent_span: TextRange,
    str: &'a str,
    events: &mut Events,
    from: TextSize,
) -> NodeBuilder<'s, 'c, Token>
where
    Token: TokenKind<'s>,
    Token::Extras: From<HashCount> + Into<HashCount>,
    Events: PeekableIterator<Item = (Event<'a>, TextRange)>
{
    let (event, span) = match events.next() {
        Some(s) => s,
        None => return builder,
    };
    match event {
        Event::Start(tag) => {
            if span != parent_span {
                builder.parse(node_once(|builder| {
                    parse_start(builder.node(), span, &tag, str, events, from)
                    .finish()
                }))
            }
            else {
                parse_start(builder, span, &tag, str, events, from)
            }
        }
        Event::Text(cow) => {
            build_token(builder, cow.to_string(), Nodes::Md_Text)
        }
        Event::Code(cow) => {
            //let ctx = Context::default();
            /*
            builder.parse_mode(
                &ctx,
                node(move |builder: &mut NodeBuilder<NeuToken>| {
                    let saved = builder.state_mut().lexer_mut().input().clone();

                    builder.state_mut().lexer_mut().input_mut().set_range(range);
                    builder.name(Nodes::Virtual);
                    builder.name(Nodes::Interpolated);
                    builder.parse(crate::parsers::neu::parser());

                    *builder.state_mut().lexer_mut().input_mut() = saved;
                }),
            );
            */
            //todo!();
            builder
        }
        Event::Html(cow) => {
            build_token(builder, cow.to_string(), Nodes::Md_Html)
        }
        Event::FootnoteReference(_span) => todo!("FootnoteReference"),
        Event::SoftBreak => {
            build_token(builder, &str[span], Nodes::Md_SoftBreak)
        }
        Event::HardBreak => {
            build_token(builder, &str[span], Nodes::Md_HardBreak)
        }
        Event::Rule => {
            build_token(builder, &str[span], Nodes::Md_Rule)
        }
        Event::TaskListMarker(_bool) => todo!("TaskListMarker"),
        _ => builder
    }
}


fn parse_start<'s, 'c, 'a, Token>(
    mut builder: NodeBuilder<'s, 'c, Token>,
    span: TextRange,
    tag: &Tag,
    str: &'a str,
    events: &mut impl PeekableIterator<Item = (Event<'a>, TextRange)>,
    from: TextSize,
) -> NodeBuilder<'s, 'c, Token>
where
    Token: TokenKind<'s>,
    Token::Extras: From<HashCount> + Into<HashCount>,
{
    //builder.set_span(span);
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
        Tag::CodeBlock(lang_kind) => {
            let mut range: Option<TextRange> = None;
            let lang_str = match lang_kind {
                CodeBlockKind::Indented => "",
                CodeBlockKind::Fenced(lang) => lang.as_ref(),
            };
            if lang_str == "neu" || lang_str == "" {
                while let Some((peeked, peeked_range)) = events.peek() {
                    if let Event::End(_) = peeked {
                        events.next();
                        break;
                    }
                    let peeked_range = *peeked_range;
                    events.next();
                    match range.as_mut() {
                        None => {
                            range = Some(peeked_range);
                        }
                        Some(range) => {
                            *range = range.cover(peeked_range);
                        }
                    }
                }
            }

            match (range, lang_kind) {
                (None, CodeBlockKind::Fenced(lang)) => {
                    //let lang_range = get_range(str, lang, span, from);
                    /*
                    builder.parse(|builder| {
                        builder.name(Nodes::Md_CodeBlockLang);
                        builder.set_span(lang_range);
                    });
                    */
                    Nodes::Md_CodeBlock
                }
                (None, CodeBlockKind::Indented) => Nodes::Md_CodeBlock,
                (Some(range), _) => {
                    //let ctx = Context::default();
                    /*
                    return builder.parse_mode(
                        &ctx,
                        move |builder: Builder<'s, NeuToken>| {
                            let saved = builder.state_mut().lexer_mut().input().clone();

                            builder.state_mut().lexer_mut().input_mut().set_range(range);
                            builder.name(Nodes::Virtual);
                            builder.name(Nodes::Interpolated);
                            builder.parse(crate::parsers::neu::parser());

                            *builder.state_mut().lexer_mut().input_mut() = saved;
                        },
                    );
                    */
                    Nodes::Md_CodeBlock
                }
            }
        }
        Tag::List(None) => Nodes::Md_UnorderedList,
        Tag::List(Some(1)) => Nodes::Md_OrderedList,
        Tag::List(_offset) => {
            //TODO: OrderedList
            Nodes::Md_OrderedList
        }
        Tag::Item => Nodes::Md_ListItem,
        Tag::FootnoteDefinition(_) => todo!("FootnoteDefinition"),
        Tag::Table(_) => todo!("Table"),
        Tag::TableHead => todo!("TableHead"),
        Tag::TableRow => todo!("TableRow"),
        Tag::TableCell => todo!("TableCell"),
        Tag::Strong => Nodes::Md_Strong,
        Tag::Strikethrough => todo!("Strikethrough"),
        Tag::Link(link_type, url, title) => {
            //let url_range = get_range(str, url, span, from);
            //let title_range = get_range(str, title, span, from);
            /*
            builder.parse(|builder| {
                builder.name(Nodes::Md_LinkUrl);
                builder.set_span(url_range);
            });
            builder.parse(|builder| {
                builder.name(Nodes::Md_LinkTitle);
                builder.set_span(title_range);
            });
            builder.name(Nodes::Md_Link);
            */
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
        }
        Tag::Image(link_type, src, title) => {
            //let src_range = get_range(str, src, span, from);
            //let title_range = get_range(str, title, span, from);
            /*
            builder.parse(|builder| {
                builder.name(Nodes::Md_ImageSrc);
                builder.set_span(src_range);
            });
            builder.parse(|builder| {
                builder.name(Nodes::Md_ImageTitle);
                builder.set_span(title_range);
            });
            builder.name(Nodes::Md_Image);
            */
            match link_type {
                LinkType::Inline => Nodes::Md_InlineImage,
                LinkType::Reference => Nodes::Md_ReferenceImage,
                LinkType::ReferenceUnknown => todo!("LinkType LinkType::ReferenceUnknown"),
                LinkType::Collapsed => todo!("LinkType LinkType::Collapsed"),
                LinkType::CollapsedUnknown => todo!("LinkType LinkType::CollapsedUnknown"),
                LinkType::Shortcut => Nodes::Md_ShortcutImage,
                LinkType::ShortcutUnknown => todo!("LinkType LinkType::ShortcutUnknown"),
                LinkType::Autolink => Nodes::Md_AutoImage,
                LinkType::Email => Nodes::Md_EmailImage,
            }
        }
    };
    builder = builder.name(name);
    while let Some((peeked, _)) = events.peek() {
        if let Event::End(_) = peeked {
            events.next();
            break;
        }
        builder = parse_event(builder, span,
                              str, events, from);
    }
    builder
}

fn build_token<'s, 'c, Token>(mut builder: NodeBuilder<'s, 'c, Token>, value: impl Into<SmolStr>, name: Name)
                              -> NodeBuilder<'s, 'c, Token>
    where
    Token: TokenKind<'s>,
    Token::Extras: From<HashCount> + Into<HashCount>,
{
    let green = builder.state_mut().cache().token(name, value);
    builder.add(Some(green))
}
