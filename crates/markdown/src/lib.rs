use text_size::{TextRange, TextSize, TextSized};
use pulldown_cmark::{Event as MdEvent, Tag as MdTag, CodeBlockKind as MdCodeBlockKind, LinkType, Alignment, CowStr, Parser};
use std::convert::{TryFrom, TryInto};

#[derive(Debug)]
pub enum Span {
    Owned(Box<str>),
    Borrowed(TextRange)
}

impl Span {
    fn offset(a: &str, orig: &str) -> usize {
        let a = a.as_ptr() as usize;
        let orig = orig.as_ptr() as usize;
        a - orig
    }

    pub fn transform(range: TextRange, span: CowStr, orig: &str, from: TextSize) -> Self {
        match span {
            CowStr::Boxed(s) => Self::Owned(s),
            CowStr::Borrowed(s) => {
                let offset: TextSize = Self::offset(s, orig).try_into().unwrap();
                let range = TextRange(offset + from, offset + s.text_size() + from);
                Self::Borrowed(range)
            },
            _ => Self::Borrowed(range)
        }
    }
    pub fn as_borrowed(&self) -> Option<TextRange> {
        match self {
            Self::Borrowed(r) => Some(*r),
            Self::Owned(_) => None,
        }
    }
}

#[derive(Debug)]
pub enum CodeBlockKind {
    Indented,
    Fenced(Span),
}

impl CodeBlockKind {
    pub fn transform(range: TextRange, kind: MdCodeBlockKind, orig: &str, from: TextSize) -> Self {
        match kind {
            MdCodeBlockKind::Indented => CodeBlockKind::Indented,
            MdCodeBlockKind::Fenced(span) => CodeBlockKind::Fenced(Span::transform(range, span, orig, from)),
        }
    }
}

#[derive(Debug)]
pub enum Tag {
    Paragraph,
    Heading(u32),
    BlockQuote,
    CodeBlock(CodeBlockKind),
    List(Option<u64>),
    Item,
    FootnoteDefinition(Span),
    Table(Vec<Alignment>),
    TableHead,
    TableRow,
    TableCell,
    Emphasis,
    Strong,
    Strikethrough,
    Link(LinkType, Span, Span),
    Image(LinkType, Span, Span),
}

impl Tag {
    pub fn transform(range: TextRange, tag: MdTag, orig: &str, from: TextSize) -> Self {
        match tag {
            MdTag::Paragraph => Tag::Paragraph,
            MdTag::Heading(lvl) => Tag::Heading(lvl),
            MdTag::BlockQuote => Tag::BlockQuote,
            MdTag::CodeBlock(c) => Tag::CodeBlock(CodeBlockKind::transform(range, c, orig, from)),
            MdTag::List(o) => Tag::List(o),
            MdTag::Item => Tag::Item,
            MdTag::FootnoteDefinition(span) => Tag::FootnoteDefinition(Span::transform(range, span, orig, from)),
            MdTag::Table(a) => Tag::Table(a),
            MdTag::TableHead => Tag::TableHead,
            MdTag::TableRow => Tag::TableRow,
            MdTag::TableCell => Tag::TableCell,
            MdTag::Emphasis => Tag::Emphasis,
            MdTag::Strong => Tag::Strong,
            MdTag::Strikethrough => Tag::Strikethrough,
            MdTag::Link(ltype, span, span2) => Tag::Link(ltype,
                                                         Span::transform(range, span, orig, from),
                                                         Span::transform(range, span2, orig, from)),
            MdTag::Image(ltype, span, span2) => Tag::Link(ltype,
                                                          Span::transform(range, span, orig, from),
                                                          Span::transform(range, span2, orig, from)),
        }
    }
}

#[derive(Debug)]
pub enum Event {
    Start(Tag),
    End(Tag),
    Text(Span),
    Code(Span),
    Html(Span),
    FootnoteReference(Span),
    SoftBreak,
    HardBreak,
    Rule,
    TaskListMarker(bool)

}

impl Event {
    pub fn transform(range: TextRange, event: MdEvent, orig: &str, from: TextSize) -> Self {
        match event {
            MdEvent::Start(tag) => Event::Start(Tag::transform(range, tag, orig, from)),
            MdEvent::End(tag) => Event::End(Tag::transform(range, tag, orig, from)),

            MdEvent::Text(span) => Event::Text(Span::transform(range, span, orig, from)),
            MdEvent::Code(span) => Event::Code(Span::transform(range, span, orig, from)),
            MdEvent::Html(span) => Event::Html(Span::transform(range, span, orig, from)),
            MdEvent::FootnoteReference(span) => Event::FootnoteReference(Span::transform(range, span, orig, from)),

            MdEvent::SoftBreak => Event::SoftBreak,
            MdEvent::HardBreak => Event::HardBreak,
            MdEvent::Rule => Event::Rule,
            MdEvent::TaskListMarker(b) => Event::TaskListMarker(b),
        }
    }
}

pub fn transform_md<'a>(parser: Parser<'a>, from: TextSize, orig: &'a str) -> impl Iterator<Item = (Event, TextRange),> + 'a {
    parser.into_offset_iter().map(move |(event, range)| {
        let range =  TextRange(
            TextSize::try_from(range.start).unwrap() + from,
            TextSize::try_from(range.end).unwrap() + from
        );
        (Event::transform(range, event, orig, from), range)
    })
}

