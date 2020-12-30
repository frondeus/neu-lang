use std::convert::TryFrom;

use crate::lexers::md_string::Token as MdStrToken;
use crate::Nodes;
use microtree_parser::*;
use microtree_parser::parsers::*;
use pulldown_cmark::{CodeBlockKind, CowStr, Event as MdEvent, LinkType, Tag};
use text_size::{TextLen, TextRange, TextSize};

pub(crate) fn inner_md_string<S: Sink>() -> impl Parser<MdStrToken, S>
{
    parse(|s| {
        s.peek()
            .at(MdStrToken::Text)
            .parse(markdown())
            .ignore_unexpected()
    })
}

pub(crate) fn markdown<S>() -> impl Parser<MdStrToken, S> + Clone
where
    //for<'s> <Token as Logos<'s>>::Extras: From<HashCount> + Into<HashCount>,
    S: Sink,
{
    parse(|mut s| {
        let next = s.lexer_mut().next().unwrap();
        let from = next.range.start();
        let value = next.value;
        let value_len: TextSize = (value.len() as u32).into();


        let mut events = pulldown_cmark::Parser::new(&value)
            .into_offset_iter()
            .map(|(event, range)| {
                let range = TextRange::new(
                    TextSize::try_from(range.start).unwrap(),
                    TextSize::try_from(range.end).unwrap(),
                );
                (event, range)
            })
            .peekable();

        s = s.alias(Nodes::Md_Value)
            .unfinished();
        let mut state = MdState {
            prev: TextRange::up_to(value_len),
            ..Default::default()
        };
        while events.peek().is_some() {
            let (next, range) = events.next().unwrap();

            if state.code {
                s = translate_code(&mut state, next, range, &value, from, s);
            }
            else {
                s = translate_event(&mut state, next, range, &value, from, s);
            }

            state.prev = range;
        }
        s.abort()
    })
}


#[derive(Default)]
struct MdState {
    prev: TextRange,
    code: bool,
    code_range: Option<TextRange>
}

type B<'c, 's, S> = Builder<'c, 's, MdStrToken, S>;

fn translate_code<'c, 's, S: Sink>(
    state: &mut MdState,
    next: MdEvent<'_>,
    range: TextRange,
    value: &str,
    from: TextSize,
    mut s: B<'c, 's, S>
) -> B<'c, 's, S> {
    if let MdEvent::End(_) = next {

        state.code = false;
        let code_range = state.code_range.take();
        if let Some(range) = code_range {
            s = s
                .alias(Nodes::Interpolated)
                .with_range(range, with_mode(crate::parsers::neu::parser()));
        }
        s = leading(state, range, value, s);
    }
    else {
        s = leading(state, range, value, s);
        let mut code_range = *state.code_range.get_or_insert(range + from);
        code_range = TextRange::cover(code_range, range + from);
        state.code_range = Some(code_range);
    }
    s
}

fn leading<'c, 's, S>(
    state: &mut MdState,
    range: TextRange,
    value: &str,
    s: B<'c, 's, S>
) -> B<'c, 's, S>
where S: Sink {
    /*

    Three scenarios:
    1. When it intersects in the beginning:
    | s-paragraph |
         |  text  |
    =
    |----|

    2. When there is leading between:
    | foo |  | bar |
    =
          |--|

    3. When there is leading before end:
    | text |
    | e-paragraph |
    =
           |------|
    */

    let leading = if state.prev.start() < range.start() {
        if state.prev.end() >= range.end() { //1.
            Some(TextRange::new(state.prev.start(), range.start()))
        }
        else if state.prev.end() < range.start() { //2.
            Some(TextRange::new(state.prev.end(), range.start()))
        } else{
            None
        }
    }
    else if state.prev.end() < range.end() {
        // 3.
        Some(TextRange::new(state.prev.end(), range.end()))
    }
    else { None }
    ;

    if let Some(leading) = leading {
        token(s, Nodes::Token, &value[leading])
    }
    else {
        s
    }
}

fn translate_event<'c, 's, S>(
    state: &mut MdState,
    next: MdEvent<'_>,
    range: TextRange,
    value: &str,
    from: TextSize,
    mut s: B<'c, 's, S>) -> B<'c, 's, S>
where
    S: Sink,
{
    s = leading(state, range, value, s);
    s = match next {
        MdEvent::Start(tag) => translate_start(state, tag, s),
        MdEvent::End(tag) => translate_end(tag, s),
        MdEvent::Text(v) => token(s, Nodes::Md_Text, v.to_string()),
        MdEvent::Html(v) => token(s, Nodes::Md_Html, v.to_string()),
        MdEvent::SoftBreak => token(s, Nodes::Md_SoftBreak, &value[range]),
        MdEvent::HardBreak => token(s, Nodes::Md_HardBreak, &value[range]),
        MdEvent::Rule => token(s, Nodes::Md_Rule, &value[range]),
        MdEvent::Code(v) => {
            let start = range.start() + TextSize::from(1);
            let end = range.end() - TextSize::from(1);
            let range = TextRange::new(start + from, end + from);
            s.alias(Nodes::Interpolated)
            .with_range(range, with_mode(crate::parsers::neu::parser()))
        }
        MdEvent::TaskListMarker(_) => todo!(),
        MdEvent::FootnoteReference(_) => todo!(),
    };

    s
}

fn token<'c, 's, S>(mut s: B<'c, 's, S>, name: microtree::Name, value: impl Into<SmolStr>)
    -> B<'c, 's, S>
where
    S: Sink
{
    s.sink_mut().event(Event::Token(name.into(), value.into() ));
    s
}

fn translate_start<'c, 's, S: Sink>(state: &mut MdState, tag: Tag, mut s: B<'c, 's, S>) -> B<'c, 's, S> {
    match tag {
        Tag::Paragraph => {
            s.start(Nodes::Md_Paragraph)
        }
        Tag::Emphasis => {
            s.start(Nodes::Md_Emphasis)
        }
        Tag::Strong => {
            s.start(Nodes::Md_Strong)
        }
        Tag::Heading(lvl) => {
            let name = match lvl {
                1 => Nodes::Md_H1,
                2 => Nodes::Md_H2,
                3 => Nodes::Md_H3,
                4 => Nodes::Md_H4,
                5 => Nodes::Md_H5,
                _ => Nodes::Md_H6,
            };
            s.start(name)
        }
        Tag::BlockQuote => {
            s.start(Nodes::Md_BlockQuote)
        }
        Tag::List(None) => s.start(Nodes::Md_UnorderedList),
        Tag::List(Some(1)) => s.start(Nodes::Md_OrderedList),
        Tag::List(_offset) => s.start(Nodes::Md_OrderedList),
        Tag::Item => s.start(Nodes::Md_ListItem),
        Tag::Link(link_type, _url, _title) => {
            s.alias(Nodes::Md_Link)
             .start(match link_type {
                LinkType::Inline =>  Nodes::Md_InlineLink,
                LinkType::Reference =>  Nodes::Md_ReferenceLink,
                LinkType::Shortcut =>  Nodes::Md_ShortcutLink,
                LinkType::Autolink =>  Nodes::Md_AutoLink,
                LinkType::Email =>  Nodes::Md_EmailLink,
                lt => todo!("LinkType: {:?}", lt)
            })
        }
        Tag::Image(link_type, _src, _title) => {
            s.alias(Nodes::Md_Image)
             .start(match link_type {
                LinkType::Inline =>  Nodes::Md_InlineImage,
                LinkType::Reference =>  Nodes::Md_ReferenceImage,
                LinkType::Shortcut =>  Nodes::Md_ShortcutImage,
                LinkType::Autolink =>  Nodes::Md_AutoImage,
                LinkType::Email =>  Nodes::Md_EmailImage,
                lt => todo!("LinkType: {:?}", lt)
            })
        }
        Tag::CodeBlock(lang_kind) => {
            let lang_str = match lang_kind {
                CodeBlockKind::Indented => "",
                CodeBlockKind::Fenced(ref lang) => lang.as_ref()
            };
            if lang_str == "neu" || lang_str == "" {
                state.code = true;
                state.code_range = None;
                s
            }
            else {
                s = s.start(Nodes::Md_CodeBlock);
                if lang_str != "" {
                    s = token(s, Nodes::Md_CodeBlockLang, lang_str.to_string());
                }
                s
            }
        }
        Tag::FootnoteDefinition(_) => {s}
        Tag::Table(_) => {s}
        Tag::TableHead => {s}
        Tag::TableRow => {s}
        Tag::TableCell => {s}
        Tag::Strikethrough => {s}
    }
}

fn translate_end<'c, 's, S: Sink>(tag: Tag, mut s: B<'c, 's, S>) -> B<'c, 's, S> {
    match tag {
        Tag::Link(_, url, title) => {
            s = token(s, Nodes::Md_LinkUrl, url.to_string());
            token(s, Nodes::Md_LinkTitle, title.to_string())
        },
        Tag::Image(_, src, title) => {
            s = token(s, Nodes::Md_ImageSrc, src.to_string());
            token(s, Nodes::Md_ImageTitle, title.to_string())
        },
        _ => s
    }
    .end()
}
