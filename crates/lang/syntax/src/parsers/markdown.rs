use std::convert::TryFrom;

use crate::lexers::md_string::Token as MdStrToken;
use crate::Nodes;
use microtree_parser::*;
use microtree_parser::parsers::*;
use pulldown_cmark::{CodeBlockKind, Event, LinkType, Tag, CowStr, HeadingLevel};
use text_size::{TextRange, TextSize};
use std::collections::BTreeMap;

pub(crate) fn inner_md_string<S: Sink>() -> impl Parser<MdStrToken, S>
{
    parse(|s| {
        s.peek()
         .at(MdStrToken::Text)
         .parse(parse(|s| {
            s.start(Nodes::Markdown)
            .parse(markdown())
            .end()
         }))
         .ignore_unexpected()
    })
}

pub(crate) fn markdown<T, S>() -> impl Parser<T, S>
where
    T: TokenKind,
    S: Sink,
    T::Extras: Into<crate::HashCount> + From<crate::HashCount>
{
    parse(|mut s| {
        let next = s.lexer_mut().next().unwrap();
        let value = next.value;
        let range = next.range;
        s.with_mode(markdown_inner(value, range, Default::default()))
    })
}

pub(crate) fn markdown_inner<S: Sink>(value: SmolStr, range: TextRange, mut to_insert: BTreeMap<TextSize, TmpSink>)
                                -> impl Parser<MdStrToken, S> {
    parse(move |s| {
        let from = range.start();
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

        let mut parser = MdParser {
            from,
            value: &value,
            s,
            to_insert: &mut to_insert,
            opts: Options {
                gaps: vec![TextRange::up_to(value_len)],
                ..Default::default()
            }
        };
        while events.peek().is_some() {
            let (next, range) = events.next().unwrap();

            parser = parser
                .insert_before(range)
                .translate(next, range);
        }
        let eof = TextRange::new(value_len, value_len);
        parser.insert_before(eof)
              .insert_pre(eof).s
    })
}


struct MdParser<'a, 'c, 's, S: Sink> {
    s: Builder<'c, 's, MdStrToken, S>,
    to_insert: &'a mut BTreeMap<TextSize, TmpSink>,
    from: TextSize,
    value: &'a str,
    opts: Options
}

impl<'a, 'c, 's, S: Sink> MdParser<'a, 'c, 's, S> {
    fn insert_before(mut self, range: TextRange) -> Self {
        let mut to_insert: Vec<TextSize> = vec![];
        for at in self.to_insert.keys() {
            if *at <= range.start() {
                to_insert.push(*at);
            }
        }
        for at in to_insert {
            if let Some(mut sink) = self.to_insert.remove(&at) {
                self.s = self.s.insert(&mut sink);
            }
        }
        self
    }
}

#[derive(Default)]
struct Options {
    code: bool,
    code_range: Option<TextRange>,
    gaps: Vec<TextRange>,
    future: BTreeMap<TextSize, (TextRange, microtree::Name)>
}

impl Options {
    fn add_range(&mut self, range: TextRange) {
        let mut new = vec![];
        let gaps = std::mem::take(&mut self.gaps);
        self.gaps = gaps.into_iter()
                 .filter_map(|gap| {
                     /*
                      1. |        GAP      |
                         |-----| RANGE |---|

                      2. |        GAP      |
                         |-----| RANGE     |

                      3. |        GAP      |
                         | RANGE       |---|

                      4. |        GAP      |
                         |    RANGE        |

                      5. |  GAP |----------| ALways remove gap add nothing
                         | RANGE           |
                     */
                     if let Some(intersection) = gap.intersect(range) {
                         if !range.contains_range(gap) {
                             if gap.start() < intersection.start() {
                                 let left = TextRange::new(gap.start(), intersection.start());
                                 new.push(left);
                             }

                             if gap.end() > intersection.end() {
                                 let right = TextRange::new(intersection.end(), gap.end());
                                 new.push(right);
                             }
                         }
                         return None;
                     }
                     Some(gap)
                 })
            .collect::<Vec<_>>();
        self.gaps.append(&mut new);
        //eprintln!("ADDED RANGE {:?} ; gaps: {:?}", range, self.gaps);
    }

    fn gap_leading_to_end(&mut self, range: TextRange) -> Option<TextRange> {
        //eprintln!("LEADING TO END: {:?} ; gaps: {:?}", range, self.gaps);

        let leading = self.gaps
            .iter()
            .filter_map(|gap| {
                gap.intersect(range)
            })
            .next();

        //eprintln!("leading: {:?}", leading);
        if let Some(leading) = leading {
            self.add_range(leading);
        }

        leading
    }

    fn gap_leading_to_start(&mut self, range: TextRange) -> Option<TextRange> {
        //eprintln!("LEADING TO TOKEN: {:?} ; gaps: {:?}", range, self.gaps);
        let leading = self.gaps
            .iter()
            .filter(|gap| {
                gap.start() < range.start()
            })
            .filter_map(|gap| {
                gap.intersect(range).map(|i| (gap, i))
            })
            .map(|(gap, intersection)| {
                let from = gap.start();
                let end = intersection.start();
                TextRange::new(from, end)
            })
            .next();

        //eprintln!("leading: {:?}", leading);
        if let Some(leading) = leading {
            self.add_range(leading);
        }

        leading
    }

    fn gap_leading_to_token(&mut self, range: TextRange) -> Option<TextRange> {
        /*
        1. |             GAP                   |
           |---| TOKEN |                       |
        */
        //eprintln!("LEADING TO TOKEN: {:?} ; gaps: {:?}", range, self.gaps);

        let leading = self.gap_leading_to_start(range);

        self.add_range(range);

        leading
    }
}

impl<'a, 'c, 's, S: Sink> MdParser<'a, 'c, 's, S> {
    fn translate(mut self, next: Event<'a>, range: TextRange) -> Self {
        self = self.insert_pre(range);
        if self.opts.code {
            self.translate_code(next, range)
        } else {
            self.translate_event(next, range)
        }
    }

    fn insert_pre(mut self, range: TextRange) -> Self {
        let mut to_remove = vec![];
        let mut future = std::mem::take(&mut self.opts.future);
        for (i, (future_range, future_name)) in future.iter() {
            if range.start() >= future_range.start() {
                to_remove.push(*i);
                let future_name = *future_name;
                let future_range = *future_range;
                if future_name == Nodes::MdImageSrc ||
                    future_name == Nodes::MdLinkUrl {
                        //It's a reference and we need to recover label
                        if let Some(leading) = self.opts.gap_leading_to_token(future_range) {
                            let leading = &self.value[leading];
                            let len = leading.len();
                            let mut start = None;
                            for (i, c) in leading.chars().rev().enumerate() {
                                let i = len - i;
                                if c == '[' { // Bingo
                                    start = Some(i);
                                    break;
                                }
                            }
                            if let Some(start) = start {
                                self.s = self.s.add_token(Nodes::Token.into(), &leading[0..start]);
                                if let Some(end) = leading[start..].find(']') {
                                    let label = &leading[start..=end + 1];
                                    self.s = self.s.alias(Nodes::MdValue);
                                    self.s = self.s.add_token(Nodes::MdReference.into(), label);
                                    self.s = self.s.add_token(Nodes::Token.into(), &leading[end + 2..]);
                                }
                            }
                        }
                    }
                self.s = self.s.alias(Nodes::MdValue);
                self = self.token(future_name.into(), future_range);
            }
        }
        for i in to_remove {
            future.remove(&i);
        }
        self.opts.future = future;
        self
    }

    fn translate_code(mut self, next: Event<'a>, range: TextRange) -> Self {
        if let Event::End(_) = next {
            self.opts.code = false;
            let code_range = self.opts.code_range.take();
            if let Some(range) = code_range {
                self = self.token_leading(range);
                self.s = self.s.alias(Nodes::MdValue);
                self.s = self.s
                             .start(Nodes::Interpolated)
                             .with_range(range + self.from, with_mode(crate::parsers::neu::parser()))
                             .end();
                //self.opts.prev = range;
            }
            let leading = self.opts.gap_leading_to_end(range);
            if let Some(leading) = leading {
                let leading = &self.value[leading];
                self.s = self.s.add_token(Nodes::Token.into(), leading);
            }
            //self = self.leading(range);
        }
        else {
            //self = self.leading(range);
            let mut code_range = *self.opts.code_range.get_or_insert(range);
            code_range = TextRange::cover(code_range, range);
            self.opts.code_range = Some(code_range);
        }
        self
    }

    fn translate_event(mut self, next: Event<'a>, range: TextRange) -> Self {
        let leading = self.opts.gap_leading_to_start(range);
        if let Some(leading) = leading {
            let leading = &self.value[leading];
            self.s = self.s.add_token(Nodes::Token.into(), leading);
        }


        match next {
            Event::Start(tag) => {
                self.translate_start(tag)
            }
            Event::End(tag) => {
                self.translate_end(tag, range)
            },
            Event::Text(_) => {
                self.s = self.s.alias(Nodes::MdValue);
                self.token(Nodes::MdText, range)
            }
            Event::Html(_) => {
                self.s = self.s.alias(Nodes::MdValue);
                self.token(Nodes::MdHtml, range)
            }
            Event::SoftBreak => {
                self.s = self.s.alias(Nodes::MdValue);
                self.token(Nodes::MdSoftBreak, range)
            },
            Event::HardBreak => {
                self.s = self.s.alias(Nodes::MdValue);
                self.token(Nodes::MdHardBreak, range)
            } ,
            Event::Rule => {
                self.s = self.s.alias(Nodes::MdValue);
                self.token(Nodes::MdRule, range)
            } ,
            Event::Code(_) => {
                let start = range.start() + TextSize::from(1);
                let end = range.end() - TextSize::from(1);
                let range = TextRange::new(start, end);
                self = self.token_leading(range);
                let range = range + self.from;
                self.s = self.s.alias(Nodes::MdValue);
                self.s = self.s.start(Nodes::Interpolated)
                               .with_range(range, with_mode(crate::parsers::neu::parser()))
                               .end();
                self
            }
            Event::TaskListMarker(_) => todo!(),
            Event::FootnoteReference(_) => todo!(),
        }
    }

    fn translate_start(mut self, tag: Tag<'a>) -> Self {
        if let Tag::CodeBlock(lang_kind) = tag {
            return match lang_kind {
                CodeBlockKind::Indented => {
                    self.opts.code = true;
                    self.opts.code_range = None;
                    self
                },
                CodeBlockKind::Fenced(lang) => {
                    if lang.as_ref() == "neu" || lang.as_ref() == "" {
                        self.opts.code = true;
                        self.opts.code_range = None;
                        self
                    }
                    else {
                        self.s = self.s.alias(Nodes::MdValue);
                        self.s = self.s.start(Nodes::MdCodeBlock);
                        self.token_cow(Nodes::MdCodeBlockLang, lang)
                    }
                }
            }
        }
        self.s = self.s.alias(Nodes::MdValue);
        match tag {
            Tag::Paragraph => {
                self.s = self.s.start(Nodes::MdParagraph);
                self
            }
            Tag::Emphasis => {
                self.s = self.s.start(Nodes::MdEmphasis);
                self
            }
            Tag::Strong => {
                self.s = self.s.start(Nodes::MdStrong);
                self
            }
            Tag::Heading(lvl) => {
                let name = match lvl {
                    HeadingLevel::H1 => Nodes::MdH1,
                    HeadingLevel::H2 => Nodes::MdH2,
                    HeadingLevel::H3 => Nodes::MdH3,
                    HeadingLevel::H4 => Nodes::MdH4,
                    HeadingLevel::H5 => Nodes::MdH5,
                    HeadingLevel::H6 => Nodes::MdH6,
                };
                self.s = self.s.start(name);
                self
            }
            Tag::BlockQuote => {
                self.s = self.s.start(Nodes::MdBlockQuote);
                self
            }
            Tag::List(None) => {
                self.s = self.s.start(Nodes::MdUnorderedList);
                self
            }
            Tag::List(Some(1)) => {
                self.s = self.s.start(Nodes::MdOrderedList);
                self
            }
            Tag::List(_offset) => {
                self.s = self.s.start(Nodes::MdOrderedList);
                self
            }
            Tag::Item => {
                self.s = self.s.start(Nodes::MdListItem);
                self
            }
            Tag::Link(link_type, _url, _title) => {
                self.s = self.s.alias(Nodes::MdLink)
                               .start(match &link_type {
                                   LinkType::Inline =>  Nodes::MdInlineLink,
                                   LinkType::Reference(_) =>  Nodes::MdReferenceLink,
                                   LinkType::Shortcut =>  Nodes::MdShortcutLink,
                                   LinkType::Autolink =>  Nodes::MdAutoLink,
                                   LinkType::Email =>  Nodes::MdEmailLink,
                                   lt => todo!("LinkType: {:?}", lt)
                               });
                self
            }
            Tag::Image(link_type, _src, _title) => {
                self.s = self.s.alias(Nodes::MdImage)
                               .start(match &link_type {
                                   LinkType::Inline =>  Nodes::MdInlineImage,
                                   LinkType::Reference(_) =>  Nodes::MdReferenceImage,
                                   LinkType::Shortcut =>  Nodes::MdShortcutImage,
                                   LinkType::Autolink =>  Nodes::MdAutoImage,
                                   LinkType::Email =>  Nodes::MdEmailImage,
                                   lt => todo!("LinkType: {:?}", lt)
                               });
                self
            }
            _ => self
        }
    }

    fn translate_end(mut self, tag: Tag<'a>, range: TextRange) -> Self {
        self = match tag {
            Tag::Link(link_type, url, title) => {
                if let LinkType::Reference(label) = link_type {
                    if let Some(range) = self.get_range(&url) {
                        self.in_future(range, Nodes::MdLinkUrl);
                    }
                    if let Some(range) = self.get_range(&title) {
                        self.in_future(range, Nodes::MdLinkTitle);
                    }
                    self.token_cow(Nodes::MdReferenceLabel, label)
                }
                else if let LinkType::Shortcut = link_type {
                    if let Some(range) = self.get_range(&url) {
                        self.in_future(range, Nodes::MdLinkUrl);
                    }
                    if let Some(range) = self.get_range(&title) {
                        self.in_future(range, Nodes::MdLinkTitle);
                    }
                    self
                }
                else {
                    self.token_cow(Nodes::MdLinkUrl, url)
                        .token_cow(Nodes::MdLinkTitle, title)
                }

            },
            Tag::Image(link_type, src, title) => {
                if let LinkType::Reference(label) = link_type {
                    if let Some(range) = self.get_range(&src) {
                        self.in_future(range, Nodes::MdImageSrc);
                    }
                    if let Some(range) = self.get_range(&title) {
                        self.in_future(range, Nodes::MdImageTitle);
                    }
                    self.token_cow(Nodes::MdReferenceLabel, label)
                }
                else if let LinkType::Shortcut = link_type {
                    if let Some(range) = self.get_range(&src) {
                        self.in_future(range, Nodes::MdImageSrc);
                    }
                    if let Some(range) = self.get_range(&title) {
                        self.in_future(range, Nodes::MdImageTitle);
                    }
                    self
                }
                else {
                    self.token_cow(Nodes::MdImageSrc, src)
                    .token_cow(Nodes::MdImageTitle, title)
                }
            },
            _ => self
        };

        let leading = self.opts.gap_leading_to_end(range);
        if let Some(leading) = leading {
            let leading = &self.value[leading];
            self.s = self.s.add_token(Nodes::Token.into(), leading);
        }
        self.s = self.s.end();
        self
    }

    fn in_future(&mut self, range: TextRange, name: microtree::Name) {
        self.opts.future.insert(range.start(), (range, name));
    }

    fn token_leading(mut self, range: TextRange) -> Self {
        let leading = self.opts.gap_leading_to_token(range);
        if let Some(leading) = leading {
            let leading = &self.value[leading];
            self.s = self.s.add_token(Nodes::Token.into(), leading);
        }
        self
    }


    fn token(mut self, name: microtree::Name, range: TextRange) -> Self {
        self = self.token_leading(range);
        let value = &self.value[range];
        self.s = self.s.add_token(name.into(), value);
        self
    }

    fn token_cow(mut self, name: microtree::Name, cow: CowStr<'a>) -> Self {
        if let Some(range) = self.get_range(&cow) {
            self = self.token_leading(range);
        }
        if !cow.is_empty() {
            self.s = self.s.add_token(name.into(), cow.to_string());
        }
        self
    }


    fn get_range(&self, cow: &CowStr<'a>) -> Option<TextRange> {
        match cow {
            CowStr::Borrowed(s) => {
                let offset = offset(s, self.value)?;
                let offset = TextSize::try_from(offset).unwrap();
                Some(TextRange::at(offset, s.text_len()))
            },
            _ => None
        }
    }
}

fn offset(a: &str, orig: &str) -> Option<usize> {
    let a = a.as_ptr() as usize;
    let orig = orig.as_ptr() as usize;
    if a >= orig { Some (a - orig) }
    else { None }
}
