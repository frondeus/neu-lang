use text_size::{TextRange, TextSize, TextSized};
use pulldown_cmark::{Event as MdEvent, Tag as MdTag, CodeBlockKind as MdCodeBlockKind, LinkType, Alignment, CowStr, Parser};
use std::ops::Range;
use std::convert::TryFrom;

#[derive(Debug)]
pub enum Span {
    Owned(Box<str>),
    Borrowed(TextRange)
}

impl Span {
    pub fn transform(range: TextRange, span: CowStr) -> Self {
        match span {
            CowStr::Boxed(s) => Self::Owned(s),
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
    pub fn transform(range: TextRange, kind: MdCodeBlockKind) -> Self {
        match kind {
            MdCodeBlockKind::Indented => CodeBlockKind::Indented,
            MdCodeBlockKind::Fenced(span) => CodeBlockKind::Fenced(Span::transform(range, span)),
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
    pub fn transform(range: TextRange, tag: MdTag) -> Self {
        match tag {
            MdTag::Paragraph => Tag::Paragraph,
            MdTag::Heading(lvl) => Tag::Heading(lvl),
            MdTag::BlockQuote => Tag::BlockQuote,
            MdTag::CodeBlock(c) => Tag::CodeBlock(CodeBlockKind::transform(range, c)),
            MdTag::List(o) => Tag::List(o),
            MdTag::Item => Tag::Item,
            MdTag::FootnoteDefinition(span) => Tag::FootnoteDefinition(Span::transform(range, span)),
            MdTag::Table(a) => Tag::Table(a),
            MdTag::TableHead => Tag::TableHead,
            MdTag::TableRow => Tag::TableRow,
            MdTag::TableCell => Tag::TableCell,
            MdTag::Emphasis => Tag::Emphasis,
            MdTag::Strong => Tag::Strong,
            MdTag::Strikethrough => Tag::Strikethrough,
            MdTag::Link(ltype, span, span2) => Tag::Link(ltype, Span::transform(range.clone(), span), Span::transform(range, span2)),
            MdTag::Image(ltype, span, span2) => Tag::Link(ltype, Span::transform(range.clone(), span), Span::transform(range, span2)),
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
    pub fn transform(range: TextRange, event: MdEvent) -> Self {
        dbg!(&range);
        match dbg!(event) {
            MdEvent::Start(tag) => Event::Start(Tag::transform(range, tag)),
            MdEvent::End(tag) => Event::End(Tag::transform(range, tag)),

            MdEvent::Text(span) => Event::Text(Span::transform(range, span)),
            MdEvent::Code(span) => Event::Code(Span::transform(range, span)),
            MdEvent::Html(span) => Event::Html(Span::transform(range, span)),
            MdEvent::FootnoteReference(span) => Event::FootnoteReference(Span::transform(range, span)),

            MdEvent::SoftBreak => Event::SoftBreak,
            MdEvent::HardBreak => Event::HardBreak,
            MdEvent::Rule => Event::Rule,
            MdEvent::TaskListMarker(b) => Event::TaskListMarker(b),
        }
    }
}

pub fn transform_md<'a>(parser: Parser<'a>, from: TextSize) -> impl Iterator<Item = (Event, TextRange),> + 'a {
    parser.into_offset_iter().map(move |(event, range)| {
        let range =  TextRange(
            TextSize::try_from(range.start).unwrap() + from,
            TextSize::try_from(range.end).unwrap() + from
        );
        (Event::transform(range, event), range)
    })
}

