#![feature(associated_type_bounds)]

mod name {
    use std::fmt::{Debug, Display};

    #[derive(PartialEq, Eq, Clone, Copy, Default, PartialOrd, Ord)]
    pub struct Name(pub(crate) &'static str);

    impl Name {
        pub const fn new(s: &'static str) -> Self {
            Self(s)
        }
        pub fn is_empty(&self) -> bool {
            self.0.is_empty()
        }
    }

    impl Display for Name {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            Display::fmt(&self.0, f)
        }
    }
    impl Debug for Name {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            Debug::fmt(&self.0, f)
        }
    }
}

mod event {
    use crate::Name;
    use smol_str::SmolStr;

    #[derive(Debug, Clone, Copy)]
    pub enum TriviaKind {
        Leading,
        Trailing,
    }
    #[derive(Debug, Clone)]
    pub enum Event {
        Start(Name),
        Alias(Name),
        Token(Name, SmolStr),
        Unfinished,
        Abort,
        Finish(Name),
        End,
        Trivia(TriviaKind, SmolStr),
        IgnoreTrivia, // Sometimes trivia is just wrong. Hack!
        Eof
    }

    impl Event {
        pub fn start(name: &'static str) -> Self {
            Self::Start(Name::new(name))
        }
        pub fn alias(name: &'static str) -> Self {
            Self::Alias(Name::new(name))
        }
        pub fn token(name: &'static str, value: impl Into<SmolStr>) -> Self {
            Self::Token(Name::new(name), value.into())
        }
        pub fn finish(name: &'static str) -> Self {
            Self::Finish(Name::new(name))
        }
        pub fn leading_trivia(value: impl Into<SmolStr>) -> Self {
            Self::Trivia(TriviaKind::Leading, value.into())
        }
        pub fn trailing_trivia(value: impl Into<SmolStr>) -> Self {
            Self::Trivia(TriviaKind::Trailing, value.into())
        }
    }
}

mod error {
    pub type Error = String;
}

mod sink {
    use crate::{Error, Event};

    pub trait Sink {
        fn event(&mut self, event: Event);
        fn error(&mut self, error: Error);
    }

    pub trait InsertableSink: Sink {
        fn finish(&mut self, sink: &mut impl Sink);
    }
}

mod sinks {
    mod wrapper_sink {
        use crate::Sink;

        pub struct WrapperSink<'s, S: Sink> {
            sink: &'s mut S
        }

        impl<'s, S: Sink> WrapperSink<'s, S> {
            pub fn new(sink: &'s mut S) -> Self {
                Self { sink }
            }
        }

        impl<'s, S: Sink> Sink for WrapperSink<'s, S> {
            fn event(&mut self, event: crate::Event) {
                self.sink.event(event);
            }

            fn error(&mut self, error: crate::Error) {
                self.sink.error(error);
            }
        }
    }

    mod green_sink {
        use microtree::{Cache, Green, Name as GreenName};

        use crate::{Error, Event, Name, Sink, SmolStr, TriviaKind};

        impl From<Name> for GreenName {
            fn from(gn: Name) -> Self {
                Self::new(gn.0)
            }
        }

        impl From<GreenName> for Name {
            fn from(gn: GreenName) -> Self {
                Self::new(gn.value())
            }
        }

        #[derive(Debug, Eq, PartialEq)]
        pub struct ParseResult {
            pub root: Green,
            pub errors: Vec<Error>
        }

        #[derive(Default)]
        pub struct GreenSink {
            cache: Cache,
            stack: Vec<UnsealedGreen>,
            roots: Vec<Green>,
            errors: Vec<Error>,
            leading: Option<SmolStr>,
            trailing: Option<SmolStr>,
        }

        impl GreenSink {
            pub fn finish(mut self) -> ParseResult {
                //assert!(self.stack.is_empty());
                let name = GreenName::new("Root");
                let root = match self.roots.len() {
                    0 | 1 => self.cache.alias(name, self.roots.into_iter().next()),
                    _ => self.cache.node(name, self.roots),
                };
                ParseResult {
                    root,
                    errors: self.errors
                }
            }

            fn current(&mut self) -> Option<&mut UnsealedGreen> {
                self.stack.last_mut()
            }

            fn add(&mut self, green: Green) {
                match self.current() {
                    Some(parent) => parent.children.push(green),
                    None => self.roots.push(green),
                }
            }

            fn add_many(&mut self, mut green: Vec<Green>) {
                match self.current() {
                    Some(parent) => parent.children.append(&mut green),
                    None => self.roots.append(&mut green),
                }
            }
        }

        #[derive(Default)]
        struct UnsealedGreen {
            names: Vec<Name>,
            children: Vec<Green>,
            aliased: bool,
            unfinished: bool
        }

        impl Sink for GreenSink {
            fn error(&mut self, error: Error) {
                self.errors.push(error);
            }

            fn event(&mut self, event: Event) {
                match event {
                    Event::Start(name) => match self.current() {
                        Some(prev) if prev.aliased => {
                            prev.names.push(name);
                            prev.aliased = false;
                        }
                        _ => {
                            self.stack.push(UnsealedGreen {
                                names: vec![name],
                                ..Default::default()
                            });
                        }
                    },
                    Event::Unfinished => match self.current() {
                        Some(prev) if prev.aliased => {
                            prev.aliased = false;
                            prev.unfinished = true;
                        }
                        _ => {
                            self.stack.push(UnsealedGreen {
                                names: vec![],
                                unfinished: true,
                                ..Default::default()
                            });
                        }
                    },
                    Event::Alias(name) => match self.current() {
                        Some(prev) if prev.aliased => {
                            prev.names.push(name);
                        }
                        _ => {
                            self.stack.push(UnsealedGreen {
                                names: vec![name],
                                aliased: true,
                                ..Default::default()
                            });
                        }
                    },
                    Event::Abort => {
                        let current = self.stack.pop().expect("Unmatched End");
                        let aliases = current.names;
                        let children = current
                            .children
                            .into_iter()
                            .map(|mut green| {
                                for alias in aliases.iter().rev() {
                                    green = self.cache.alias(alias.clone().into(), green);
                                }
                                green
                            })
                            .collect();
                        self.add_many(children);
                    }
                    Event::Finish(name) => {
                        let current = self.stack.pop().expect("Unmatched End");
                        let aliases = current.names.into_iter().rev();

                        let mut green = self.cache.node(name.into(), current.children);
                        for alias in aliases {
                            green = self.cache.alias(alias.into(), green);
                        }

                        self.add(green);
                    }
                    Event::End => {
                        let current = self.stack.pop().expect("Unmatched End");
                        let mut names = current.names.into_iter().rev();
                        let name = names.next().unwrap_or_default();
                        let aliases = names;

                        let mut green = self.cache.node(name.into(), current.children);
                        for alias in aliases {
                            green = self.cache.alias(alias.into(), green);
                        }

                        self.add(green);
                    }
                    Event::Token(name, value) => {
                        let aliases = match self.current() {
                            Some(prev) if prev.aliased => self.stack.pop().unwrap().names,
                            _ => {
                                vec![]
                            }
                        }
                        .into_iter()
                        .rev();

                        let leading = self.leading.take().unwrap_or_default();
                        let trailing = self.trailing.take().unwrap_or_default();
                        let mut green = self
                            .cache
                            .with_trivia(name.into(), leading, value, trailing);
                        for alias in aliases {
                            green = self.cache.alias(alias.into(), green);
                        }

                        match self.current() {
                            Some(parent) => parent.children.push(green),
                            None => self.roots.push(green),
                        }
                    }
                    Event::IgnoreTrivia => {
                        self.leading.take();
                        self.trailing.take();
                    }
                    Event::Trivia(TriviaKind::Leading, value) => {
                        self.leading.get_or_insert(value);
                    }
                    Event::Trivia(TriviaKind::Trailing, value) => {
                        self.trailing.get_or_insert(value);
                    }
                    Event::Eof => {
                        let leading = self.leading.take().unwrap_or_default();
                        let trailing = self.trailing.take().unwrap_or_default();
                        if leading != "" || trailing != "" {
                            let trivia = format!("{}{}", leading, trailing);
                            let green = self.cache.with_trivia(microtree::Name::new("eof"),
                                                               trivia, "", "");
                            match self.current() {
                                Some(parent) => parent.children.push(green),
                                None => self.roots.push(green)
                            }
                        }
                    }
                }
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn nodes_and_aliases() {
                let mut sink = GreenSink::default();
                sink.event(Event::alias("Value"));
                sink.event(Event::start("SExp"));
                sink.event(Event::token("token", "("));
                sink.event(Event::alias("Value"));
                sink.event(Event::token("atom", "foo"));
                sink.event(Event::alias("Value"));
                sink.event(Event::token("atom", "bar"));
                sink.event(Event::token("token", ")"));
                sink.event(Event::End);

                test_event(
                    sink,
                    r#"--- GREEN TREE ---
                                    Root, Value, SExp @ 0..8
                                        token @ 0..1 = `(`
                                        Value, atom @ 1..4 = `foo`
                                        Value, atom @ 4..7 = `bar`
                                        token @ 7..8 = `)`
                                    --- END ---"#,
                );
            }

            #[test]
            fn unfinished() {
                let mut sink = GreenSink::default();
                sink.event(Event::alias("Value"));
                sink.event(Event::Unfinished);
                sink.event(Event::token("token", "("));
                sink.event(Event::token("atom", "foo"));
                sink.event(Event::token("atom", "bar"));
                sink.event(Event::token("token", ")"));
                sink.event(Event::finish("SExp"));

                test_event(
                    sink,
                    r#"--- GREEN TREE ---
                                    Root, Value, SExp @ 0..8
                                        token @ 0..1 = `(`
                                        atom @ 1..4 = `foo`
                                        atom @ 4..7 = `bar`
                                        token @ 7..8 = `)`
                                    --- END ---"#,
                );
            }

            #[test]
            fn abort() {
                let mut sink = GreenSink::default();
                sink.event(Event::alias("Value"));
                sink.event(Event::Unfinished);
                sink.event(Event::token("number", "2"));
                sink.event(Event::Abort);

                test_event(
                    sink,
                    r#"--- GREEN TREE ---
                                    Root, Value, number @ 0..1 = `2`
                                    --- END ---"#,
                );
            }

            #[test]
            fn leading() {
                let mut sink = GreenSink::default();
                sink.event(Event::alias("Value"));
                sink.event(Event::Unfinished);
                sink.event(Event::leading_trivia("\n  "));
                sink.event(Event::token("number", "2"));
                sink.event(Event::Abort);

                test_event(
                    sink,
                    r#"--- GREEN TREE ---
                                    Root, Value, number @ 0..4 = `2` ; leading: `\n  `
                                    --- END ---"#,
                );
            }

            #[test]
            fn trailing() {
                let mut sink = GreenSink::default();
                sink.event(Event::alias("Value"));
                sink.event(Event::Unfinished);
                sink.event(Event::trailing_trivia("   "));
                sink.event(Event::token("number", "2"));
                sink.event(Event::Abort);

                test_event(
                    sink,
                    r#"--- GREEN TREE ---
                                    Root, Value, number @ 0..4 = `2` ; trailing: `   `
                                    --- END ---"#,
                );
            }

            // Precedence is now handled by TmpSink

            fn test_event(sink: GreenSink, expected: &'static str) {
                use diff_assert::assert_diff;

                let tree = sink.finish();
                let tree = format!("{:?}", tree.root);
                let expected = format!("{}\n", unindent::unindent(expected));

                assert_diff!(expected, tree);
            }
        }
    }

    mod test_sink {
        use crate::Sink;

        #[derive(Default, Debug)]
        pub struct TestSink {
            pub events: Vec<String>,
            pub errors: Vec<String>,
        }

        impl Sink for TestSink {
            fn event(&mut self, event: crate::Event) {
                self.events.push(format!("{:?}", event));
            }

            fn error(&mut self, error: crate::Error) {
                self.errors.push(format!("{}", error));
            }
        }

        #[cfg(test)]
        mod tests {
            use crate::Event;

            use super::*;

            #[test]
            fn works() {
                let mut sink = TestSink::default();
                sink.event(Event::alias("Value"));
                sink.event(Event::start("SExp"));
                sink.event(Event::token("token", "("));
                sink.event(Event::alias("Value"));
                sink.error("Foo".into());
                sink.event(Event::token("atom", "foo"));
                sink.event(Event::alias("Value"));
                sink.event(Event::token("atom", "bar"));
                sink.event(Event::token("token", ")"));
                sink.event(Event::End);

                use diff_assert::assert_diff;

                assert_diff!(
                    unindent::unindent(
                        r#"
                                Alias("Value")
                                Start("SExp")
                                Token("token", "(")
                                Alias("Value")
                                Token("atom", "foo")
                                Alias("Value")
                                Token("atom", "bar")
                                Token("token", ")")
                                End"#
                    ),
                    sink.events.join("\n")
                );
                assert_diff!("Foo", sink.errors.join("\n"));
            }
        }
    }

    mod write_sink {
        use std::io::Write;

        use crate::Sink;

        pub struct WriteSink<'a, EV: Write, ER: Write> {
            events: &'a mut EV,
            errors: &'a mut ER,
        }

        impl<'a, EV: Write, ER: Write> WriteSink<'a, EV, ER> {
            pub fn new(events: &'a mut EV, errors: &'a mut ER) -> Self {
                Self { events, errors }
            }
        }

        impl<'a, EV: Write, ER: Write> Sink for WriteSink<'a, EV, ER> {
            fn event(&mut self, event: crate::Event) {
                writeln!(self.events, "{:?}", event).unwrap();
            }

            fn error(&mut self, error: crate::Error) {
                writeln!(self.errors, "{}", error).unwrap();
            }
        }

        #[cfg(test)]
        mod tests {
            use crate::Event;

            use super::*;

            #[test]
            fn works() {
                let mut events: Vec<u8> = Vec::new();
                let mut errors: Vec<u8> = Vec::new();
                let mut sink = WriteSink::new(&mut events, &mut errors);
                sink.event(Event::alias("Value"));
                sink.event(Event::start("SExp"));
                sink.event(Event::token("token", "("));
                sink.event(Event::alias("Value"));
                sink.error("Foo".into());
                sink.event(Event::token("atom", "foo"));
                sink.event(Event::alias("Value"));
                sink.event(Event::token("atom", "bar"));
                sink.event(Event::token("token", ")"));
                sink.event(Event::End);

                let events = String::from_utf8_lossy(&events[..]);
                let errors = String::from_utf8_lossy(&errors[..]);

                use diff_assert::assert_diff;

                assert_diff!(
                    unindent::unindent(
                        r#"
                                Alias("Value")
                                Start("SExp")
                                Token("token", "(")
                                Alias("Value")
                                Token("atom", "foo")
                                Alias("Value")
                                Token("atom", "bar")
                                Token("token", ")")
                                End"#
                    ),
                    events
                );
                assert_diff!("Foo", errors);
            }
        }
    }

    mod dbg_sink {
        use crate::Sink;

        #[derive(Default, Debug)]
        pub struct DbgSink;

        impl Sink for DbgSink {
            fn event(&mut self, event: crate::Event) {
                println!("{:?}", event);
            }

            fn error(&mut self, error: crate::Error) {
                eprintln!("ERROR: {}", error);
            }
        }
    }

    mod tmp_sink {
        use crate::{Error, Event, InsertableSink, Sink};

        #[derive(Debug, Default, Clone)]
        pub struct TmpSink {
            pub events: Vec<Event>,
            pub errors: Vec<String>
        }

        impl TmpSink {
            pub fn append(&mut self, mut other: Self) {
                self.events.append(&mut other.events);
                self.errors.append(&mut other.errors);
            }
        }

        impl InsertableSink for TmpSink {
            fn finish(&mut self, sink: &mut impl Sink) {
                for event in std::mem::take(&mut self.events) {
                    sink.event(event);
                }
                for error in std::mem::take(&mut self.errors) {
                    sink.error(error);
                }
            }
        }

        impl Sink for TmpSink {
            fn event(&mut self, event: Event) {
                self.events.push(event);
            }

            fn error(&mut self, error: Error) {
                self.errors.push(error);
            }
        }
    }

    pub use wrapper_sink::*;
    pub use green_sink::*;
    pub use test_sink::*;
    pub use write_sink::*;
    pub use dbg_sink::*;
    pub use tmp_sink::*;
}

mod spanned {
    use std::fmt::Debug;

    use crate::{SmolStr, TextRange};

    #[derive(Clone)]
    pub struct Spanned<Tok> {
        pub token: Tok,
        pub value: SmolStr,
        pub range: TextRange,
    }

    impl<Tok: Debug> Debug for Spanned<Tok> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?} `{}`", self.token, self.value)
        }
    }
}

mod peekable {
    use std::iter::Peekable;

    pub trait PeekableIterator: Iterator {
        fn peek(&mut self) -> Option<&Self::Item>;
    }

    impl<I: Iterator> PeekableIterator for Peekable<I> {
        fn peek(&mut self) -> Option<&Self::Item> {
            Peekable::peek(self)
        }
    }
}

mod token_kind {
    use crate::Source;

    pub trait TokenKind: std::fmt::Display
        + std::fmt::Debug
        + PartialEq
        + Clone
        + Copy
        + Send
        + Sync
    {
        type Extras: Default + Clone;
        const ERROR: Self;
        fn lex(source: &mut Source<'_>, extras: &mut Self::Extras) -> Self;
        fn mergeable(self, _other: Self) -> bool {
            false
        }
    }
}

mod source {
    use std::convert::TryFrom;

    use crate::{TextRange, TextSize, TextLen, SmolStr};

    #[derive(Clone, Copy)]
    pub struct Source<'s> {
        source: &'s str,
        range: TextRange
    }

    impl<'s> Source<'s> {
        pub fn bump(&mut self, len: usize) -> TextSize {
            let end = match self
                .as_ref()
                .char_indices()
                .nth(len - 1)
                .and_then(|(last, c)| TextSize::try_from(last + c.len_utf8()).ok())
            {
                Some(last) => self.range.start() + last,
                None => self.range.end(),
            };
            self.set_cursor(end);

            end
        }

        pub fn rewind(&mut self, len: usize) {
            let range = TextRange::new(
                self.range.start().checked_sub(TextSize::try_from(4 * len).unwrap()).unwrap_or_default(),
                self.range.start(),
            );
            let end = match self.source[range]
                .char_indices()
                .rev()
                .nth(len - 1)
                .and_then(|(last, _c)| TextSize::try_from(last).ok())
            {
                Some(last) => range.start() + last,
                None => TextSize::default(),
            };
            self.set_cursor(end);
        }

        pub fn cursor(&self) -> TextSize {
            self.range.start()
        }

        fn set_cursor(&mut self, cursor: TextSize) {
            self.range = TextRange::new(cursor, self.range.end());
        }

        pub fn with_range(mut self, range: TextRange) -> Self {
            self.range = range;
            self
        }

        pub fn range_span(&self, range: TextRange) -> SmolStr {
            self.source[range].into()
        }
    }

    impl<'s> AsRef<str> for Source<'s> {
        fn as_ref(&self) -> &str {
            &self.source[self.range]
        }
    }

    impl<'s> From<&'s str> for Source<'s> {
        fn from(source: &'s str) -> Self {
            Self {
                source,
                range: TextRange::up_to(source.text_len())
            }
        }
    }
}

mod lexer {
    use crate::{PeekableIterator, Source, Spanned, TextRange, TextSize, TokenKind};

    #[derive(Clone)]
    pub struct Lexer<'s, Tok: TokenKind> {
        #[allow(clippy::option_option)]
        peeked: Option<Option<(
            Source<'s>,
            Spanned<Tok>)>>,
        source: Source<'s>,
        pub extras: Tok::Extras
        //inner: Inner<'s, Tok>,
    }

    impl<'s, Tok: TokenKind> Lexer<'s, Tok> {
        pub fn new(source: &'s str) -> Self {
            Self {
                //inner: Inner::new(source),
                source: Source::from(source),
                peeked: None,
                extras: Default::default()
            }
        }

        pub fn with_range(&self, range: TextRange) -> Self {
            Self {
                source: self.source.with_range(range),
                peeked: None,
                extras: Default::default()
            }
        }

        pub fn morph<Tok2>(self) -> Lexer<'s, Tok2>
        where
            Tok2: TokenKind,
            Tok::Extras: Into<Tok2::Extras>,
        {
            Lexer {
                peeked: None,
                source: self.source,
                extras: self.extras.into()
                //inner: self.inner.morph(),
            }
        }

        //pub fn span(&self) -> TextRange {
            //let range = self.inner.span();
            //TextRange::new((range.start as u32).into(), (range.end as u32).into())
            //todo!();
        //}

        //pub(crate) fn text_for_span(&self, span: TextRange) -> SmolStr {
            //let source = self.inner.source();
            //source[span].into()
            //todo!();
        //}

        pub fn peek_token(&mut self) -> Option<Tok> {
            self.peek().map(|t| t.token)
        }

        pub fn source(&self) -> &Source<'s> {
            &self.source
        }

        pub fn rewind(&mut self, count: usize) {
            self.source.rewind(count);
            self.peeked = None;
            /*
            let offset = self.inner.span().end - count;
            let source = self.inner.source();
            let subsource = &source[offset..];
            let mut new_inner = Inner::new(source);
            new_inner.bump(offset);
            self.inner = new_inner;
            self.peeked = None;
            */
        }
    }

    impl<'s, Tok> PeekableIterator for Lexer<'s, Tok>
    where
        Tok: TokenKind,
    {
        fn peek(&mut self) -> Option<&Self::Item> {
            if self.peeked.is_none() {
                let saved = self.source;
                let token = self.next();
                let original = std::mem::replace(&mut self.source, saved);
                self.peeked = Some(token.map(|token| (original, token)));
            }

            self.peeked
                .as_ref()
                .and_then(|t| t.as_ref())
                .map(|(_, t)| t)
        }
    }

    impl<'s, Tok: TokenKind> Iterator for Lexer<'s, Tok> {
        type Item = Spanned<Tok>;

        fn next(&mut self) -> Option<Self::Item> {
            let mut first = self.lex()?;
            loop {
                match self.peek_one() {
                    Some(token) if first.token.mergeable(token.token) => {
                        let from = first.range.start();
                        let len: TextSize = ((first.value.len() + token.value.len()) as u32).into();
                        let range = TextRange::at(from, len);
                        first.range = range;
                        let new_value = self.source.range_span(range);
                        first.value = new_value;
                        self.lex();
                    }
                    _ => break Some(first),
                }
            }
        }
    }

    impl<'s, Tok: TokenKind> Lexer<'s, Tok> {
        fn lex(&mut self) -> Option<Spanned<Tok>> {
            if let Some(peeked) = self.peeked.take() {
                if let Some((original, peeked)) = peeked {
                    self.source = original;
                    return Some(peeked);
                }
                return None;
            }
            if self.source.as_ref().is_empty() {
                return None;
            }

            let from = self.source.cursor();
            let token = Tok::lex(&mut self.source, &mut self.extras);
            let to = self.source.cursor();

            let range = TextRange::new(from, to);
            let value = self.source.range_span(range);
            Some(Spanned {
                token,
                range,
                value,
            })
        }

        fn peek_one(&mut self) -> Option<&Spanned<Tok>> {
             if self.peeked.is_none() {
                 let saved = self.source;
                 let token = self.lex();
                let original = std::mem::replace(&mut self.source, saved);
                self.peeked = Some(token.map(|token| (original, token)));
            }

            self.peeked
                .as_ref()
                .and_then(|t| t.as_ref())
                .map(|(_, t)| t)
        }
    }
}

mod callback_result {
    use crate::TokenKind;

    pub trait CallbackResult<P, T: TokenKind> {
        fn result<C: Fn(P) -> T>(self, c: C) -> T;
    }

    impl<P, T: TokenKind> CallbackResult<P, T> for P {
        fn result<C: Fn(P) -> T>(self, c: C) -> T {
            c(self)
        }
    }

    impl<T: TokenKind> CallbackResult<(), T> for bool {
        fn result<C: Fn(()) -> T>(self, c: C) -> T {
            match self {
                true => c(()),
                _ => T::ERROR
            }
        }
    }
}

mod parser {
    use crate::{Context, Sink, State, TokenKind};

    pub trait Parser<Tok, S>
    where
        Tok: TokenKind,
        S: Sink,
    {
        fn parse<'s, 'c>(
            &mut self,
            state: State<'s, Tok, S>,
            context: Context<'c, Tok>,
        ) -> State<'s, Tok, S>;
    }

    impl<Tok, S, P> Parser<Tok, S> for &mut P
    where
        Tok: TokenKind,
        S: Sink,
        P: Parser<Tok, S> + ?Sized
    {
        fn parse<'s, 'c>(
            &mut self,
            state: State<'s, Tok, S>,
            context: Context<'c, Tok>,
        ) -> State<'s, Tok, S> {
            (*self).parse(state, context)
        }
    }
}

mod context {
    use crate::{Lexer, TokenKind};

    pub trait Trivia<Tok>
    where
        Tok: TokenKind,
    {
        fn parse<'s>(&self, state: &mut Lexer<'s, Tok>);
    }

    pub struct Context<'c, Tok>
    where
        Tok: TokenKind,
    {
        pub leading: Option<&'c dyn Trivia<Tok>>,
        pub trailing: Option<&'c dyn Trivia<Tok>>,
    }

    impl<'c, Tok> Default for Context<'c, Tok>
    where
        Tok: TokenKind //+ 'c,
    {
        fn default() -> Self {
            Self {
                leading: None,
                trailing: None,
            }
        }
    }

    impl<'c, Tok> Clone for Context<'c, Tok> where Tok: TokenKind {
        fn clone(&self) -> Self {
            Self {
                leading: self.leading.clone(),
                trailing: self.trailing.clone()
            }
        }
    }
    impl<'c, Tok> Copy for Context<'c, Tok> where Tok: TokenKind {}

    impl<'c, Tok> Context<'c, Tok>
    where
        Tok: TokenKind //+ 'c,
    {
        pub fn new(trivia: &'c dyn Trivia<Tok>) -> Self {
            Self {
                leading: Some(trivia),
                trailing: Some(trivia),
            }
        }

        pub(crate) fn leading(&self) -> Option<&'c dyn Trivia<Tok>> {
            self.leading
        }

        pub(crate) fn trailing(&self) -> Option<&'c dyn Trivia<Tok>> {
            self.trailing
        }
    }
}

mod state {
    use crate::{Context, Lexer, Parser, Sink, TokenKind};

    pub struct State<'s, Tok, S>
    where
        Tok: TokenKind,
        S: Sink,
    {
        pub(crate) lexer: Lexer<'s, Tok>,
        pub(crate) sink: S,
        pub(crate) trivia: usize
    }

    impl<'s, Tok, S> State<'s, Tok, S>
    where
        Tok: TokenKind,
        S: Sink + Default,
    {
        pub fn parse(lexer: Lexer<'s, Tok>, mut parser: impl Parser<Tok, S>) -> S {
            let ctx = Context::default();
            let state = Self::new(
                lexer,
                Default::default(),
            );
            let state = parser.parse(state, ctx);
            state.sink
        }
    }

    impl<'s, Tok, S> State<'s, Tok, S>
    where
        Tok: TokenKind,
        S: Sink,
    {
        pub fn new(
            lexer: Lexer<'s, Tok>,
            sink: S,
        ) -> Self {
            Self {
                sink,
                lexer,
                trivia: 0
            }
        }

        pub fn with_sink<S2: Sink>(self, new_sink: S2) -> (State<'s, Tok, S2>, S) {
            let Self { lexer, sink, trivia } = self;
            (State {
                sink: new_sink,
                lexer,
                trivia
            }, sink)
        }

        pub fn morph<Tok2>(self) -> State<'s, Tok2, S>
        where Tok2: TokenKind,
              Tok::Extras: Into<Tok2::Extras>
        {
            let Self { lexer, sink, .. } = self;
            State {
                sink,
                lexer: lexer.morph(),
                trivia: 0
            }
        }

        pub fn count_trivia(&mut self, count: usize) {
            self.trivia += count;
        }

        pub fn reset_count_trivia(&mut self) {
            self.trivia = 0;
        }

        pub fn sink_mut(&mut self) -> &mut S {
            &mut self.sink
        }
        pub fn lexer_mut(&mut self) -> &mut Lexer<'s, Tok> {
            &mut self.lexer
        }
    }
}

mod peek {
    use itertools::Itertools;

    use crate::{Name, Parser, Sink, TokenKind, Builder};

    pub enum Peek<'c, 's, Tok, S>
    where
        Tok: TokenKind,
        S: Sink,
    {
        Found { s: Builder<'c, 's, Tok, S>, handled: bool },
        None {
            s: Builder<'c, 's, Tok, S>,
            expected: Vec<Option<Tok>>,
            peeked: Option<Tok>,
        }
    }

    impl<'c, 's, Tok, S> std::fmt::Debug for Peek<'c, 's, Tok, S>
        where
        Tok: TokenKind + std::fmt::Debug,
        S: Sink,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match &self {
                Self::Found { handled, ..} => write!(f, "Peek::Found {{ s, handled: {} }}", handled),
                Self::None { expected, peeked, .. } => {
                    write!(f, "Peek::None {{ s, expected: {:?}, peeked: {:?} }}",
                           expected,
                           peeked,
                    )
                }
            }
        }
    }

    impl<'c, 's, Tok, S> Peek<'c, 's, Tok, S>
    where
        Tok: TokenKind,
        S: Sink,
    {
        pub fn at(self, expected: impl Into<Option<Tok>>) -> Self {
            let e = expected.into();
            match self {
                Self::Found{..} => self,
                Self::None { s, peeked, .. } if e == peeked => Self::Found {s, handled: false},
                Self::None { s, peeked, mut expected } => {
                    expected.push(e);
                    Self::None { s, peeked, expected }
                }
            }
        }

        pub fn at_unexpected(self, expected: impl Into<Option<Tok>>) -> Self {
            let e = expected.into();
            match self {
                Self::Found{..} => self,
                Self::None { s, peeked, .. } if e == peeked => Self::Found { s, handled: false },
                s => s,
            }
        }

        pub fn parse(self, parser: impl Parser<Tok, S>) -> Self {
            match self {
                Self::Found{mut s, handled: false } => {
                    s = s.parse(parser);
                    Self::Found { s, handled: true}
                },
                s => s
            }
        }

        pub fn parse_else(self, parser: impl Parser<Tok, S>) -> Self {
            match self {
                Self::None {mut s, peeked, expected } => {
                    s = s.parse(parser);
                    Self::None { s, peeked, expected }
                },
                s => s
            }
        }

        pub fn parse_always(self, parser: impl Parser<Tok, S>) -> Self {
            match self {
                Self::Found { s, .. } => Self::Found { s: s.parse(parser), handled: true },
                Self::None {mut s, peeked, expected } => {
                    s = s.parse(parser);
                    Self::None { s, peeked, expected }
                },
            }
        }

        pub fn ignore_unexpected(self) -> Builder<'c, 's, Tok, S> {
            match self {
                Self::Found{s, ..} |
                Self::None { s, .. } => s
            }
        }

        pub fn expect(self) -> Builder<'c, 's, Tok, S> {
            match self {
                Self::Found{s, ..} => s,
                Self::None { mut s, expected, .. } => {
                    let next = s.state.lexer_mut().next();
                    let err_value = next
                        .as_ref()
                        .map(|e| e.value.to_string())
                        .unwrap_or_else(|| "".to_string());
                    s = s.add_token(Name::new("error"), err_value);
                    s.state.sink_mut().error(format!(
                        "Expected one of: {} but found {}",
                        expected
                            .into_iter()
                            .map(|e| e
                                .map(|e| format!("{}", e))
                                .unwrap_or_else(|| "EOF".to_string()))
                            .join(", "),
                            next
                                .as_ref()
                                .map(|e| e.token.to_string())
                                .unwrap_or_else(|| "EOF".to_string())
                            ));

                    s
                }
            }
        }
    }

}

mod builder {
    use crate::{Context, Event, InsertableSink, Lexer, Name, Parser, Peek, Sink, SmolStr, Spanned, State, TextRange, TokenKind, Trivia, TriviaKind};

    pub struct Builder<'c, 's, Tok, S>
    where
        Tok: TokenKind,
        S: Sink,
    {
        pub(crate) state: State<'s, Tok, S>,
        pub(crate) ctx: Context<'c, Tok>,
    }

    impl<'c, 's, Tok, S> Builder<'c, 's, Tok, S>
    where
        Tok: TokenKind,
        S: Sink,
    {
        pub fn peek(mut self) -> Peek<'c, 's, Tok, S> {
            let peeked = self.peek_token();
            Peek::None {
                peeked,
                expected: Default::default(),
                s: self,
            }
        }

        fn handle_trivia(
            &mut self,
            trivia: Option<&'c dyn Trivia<Tok>>,
            kind: TriviaKind,
        ) {
            if let Some(trivia) = trivia {
                let start = self.state.lexer_mut().source().cursor();
                trivia.parse(self.state.lexer_mut());
                let end = self.state.lexer_mut().source().cursor();
                if start != end {
                    let text_range = TextRange::new(start, end);
                    let value = self.state.lexer_mut().source().range_span(text_range);
                    self.state.count_trivia(value.len());
                    self.state.sink_mut().event(Event::Trivia(kind, value));
                }
            }
        }

        fn next_token(&mut self) -> Option<Spanned<Tok>> {
            let leading = self.ctx.leading();
            self.handle_trivia(leading, TriviaKind::Leading);

            let token = self.state.lexer_mut().next();

            let trailing = self.ctx.trailing();
            self.handle_trivia(trailing, TriviaKind::Trailing);

            token
        }

        pub fn sink_mut(&mut self) -> &mut S {
            self.state.sink_mut()
        }

        pub(crate) fn peek_token(&mut self) -> Option<Tok> {
            let leading = self.ctx.leading();
            self.handle_trivia(leading, TriviaKind::Leading);

            let token = self.state.lexer_mut().peek_token();

            let trailing = self.ctx.trailing();
            self.handle_trivia(trailing, TriviaKind::Trailing);

            token
        }

        pub fn lexer_mut(&mut self) -> &mut Lexer<'s, Tok> {
            self.state.lexer_mut()
        }

        pub fn alias(mut self, name: impl Into<Name>) -> Self {
            let name = name.into();
            self.state.sink_mut().event(Event::Alias(name));
            self
        }

        pub fn add_token(mut self, name: Name, value: impl Into<SmolStr>) -> Self {
            self.state.reset_count_trivia();
            self.state.sink_mut().event(Event::Token(name.into(), value.into()));
            self
        }

        pub fn token(mut self) -> Self {
            if let Some(value) = self.next_token().map(|t| t.value) {
                self.add_token(Name::new("token"), value)
            } else {
                self
            }
        }

        pub fn parse<P>(self, mut parser: P) -> Self
        where
            P: Parser<Tok, S>,
        {
            let Self { state, ctx } = self;
            let state = parser.parse(state, ctx);
            Self { state, ctx }
        }

        pub fn start(mut self, name: impl Into<Name>) -> Self {
            let name = name.into();
            self.state.sink_mut().event(Event::Start(name));
            self
        }

        pub fn expect<T>(mut self, expected: T) -> Self
        where
            T: Into<Option<Tok>>,
        {
            let expected = expected.into();
            let next = self.next_token();
            if expected == next.as_ref().map(|t| t.token) {
                return if let Some(value) = next.map(|t| t.value) {
                    self.add_token(Name::new("token"), value)
                } else {
                    self.state.reset_count_trivia();
                    self.state.sink_mut().event(Event::Eof);
                    self
                };
            }
            let err_value = next
                .as_ref()
                .map(|e| e.value.to_string())
                .unwrap_or_else(|| "".to_string());
            self = self.add_token(Name::new("error"), err_value);
            self.state.sink_mut().error(format!(
                "Expected {} but found {}",
                expected
                    .map(|e| e.to_string())
                    .unwrap_or_else(|| "EOF".to_string()),
                next
                    .as_ref()
                    .map(|e| e.token.to_string())
                    .unwrap_or_else(|| "EOF".to_string())
            ));
            self
        }

        pub fn end(mut self) -> Self {
            self.state.sink_mut().event(Event::End);
            self
        }

        pub fn unfinished(mut self) -> Self {
            self.state.sink_mut().event(Event::Unfinished);
            self
        }

        pub fn finish(mut self, name: impl Into<Name>) -> Self {
            let name = name.into();
            self.state.sink_mut().event(Event::Finish(name));
            self
        }

        pub fn abort(mut self) -> Self {
            self.state.sink_mut().event(Event::Abort);
            self
        }

        pub fn with_ctx<'c2>(self, ctx2: Context<'c2, Tok>, mut parser: impl Parser<Tok, S>) -> Self {
            let Self { state, ctx } = self;

            let state  = parser.parse(state, ctx2);

            Self { state, ctx }
        }

        pub fn with_mode<'c2, Tok2>(self, mut parser: impl Parser<Tok2, S>) -> Self
        where
            Tok2: TokenKind,
            Tok::Extras: Into<Tok2::Extras>,
            Tok2::Extras: Into<Tok::Extras>,
        {
            let Self { state, ctx } = self;

            let inner = Context::default();

            let mut state = parser.parse(state.morph(), inner);

            let trivia = state.trivia;
            if trivia > 0 {
                state.sink_mut().event(Event::IgnoreTrivia);
                state.lexer_mut().rewind(trivia);
            }

            Self { state: state.morph(), ctx }
        }

        pub fn insert(self, tmp: &mut impl InsertableSink) -> Self {
            let Self { mut state, ctx } = self;
            tmp.finish(state.sink_mut());
            Self { state, ctx }
        }

        pub fn with_sink<S2: Sink + Default>(self, mut parser: impl Parser<Tok, S2>) -> (Self, S2)
        {
            let Self { state, ctx } = self;

            let (state, prev_sink) = state.with_sink(S2::default());

            let state = parser.parse(state, ctx);

            let (state, new_sink) = state.with_sink(prev_sink);

            (Self { state, ctx }, new_sink)
        }

        pub fn with_range(self, range: TextRange, mut parser: impl Parser<Tok, S>) -> Self {
            let Self { mut state, ctx } = self;

            //let source = state.lexer_mut().source();
            //let subsource = &source[range];
            //let sublexer = Lexer::new(subsource);
            let sublexer = state.lexer_mut().with_range(range);


            let previous = std::mem::replace(state.lexer_mut(), sublexer);

            let mut state = parser.parse(state, ctx);

            let _ = std::mem::replace(state.lexer_mut(), previous);

            Self { state, ctx }
        }
    }
}

mod trivia_fn {
    use std::marker::PhantomData;

    use crate::{Lexer, TokenKind, Trivia};

    pub struct TriviaFn<F, Tok>
    where
        F: for<'s> Fn(&mut Lexer<'s, Tok>),
        Tok: TokenKind,
    {
        f: F,
        _phantom: PhantomData<Tok>,
    }

    impl<F, Tok> Trivia<Tok> for TriviaFn<F, Tok>
    where
        F: for<'s> Fn(&mut Lexer<'s, Tok>),
        Tok: TokenKind,
    {
        fn parse<'s>(&self, state: &mut Lexer<'s, Tok>) {
            (self.f)(state)
        }
    }

    pub fn trivia<F, Tok>(f: F) -> impl Trivia<Tok>
    where
        F: for<'s> Fn(&mut Lexer<'s, Tok>),
        Tok: TokenKind,
    {
        TriviaFn {
            f,
            _phantom: PhantomData,
        }
    }
}

mod parse_fn {
    use std::marker::PhantomData;

    use crate::{Builder, Context, Parser, Sink, State, TokenKind};

    pub struct ParseFn<F, Tok, S>
    where
        F: for<'c, 's> FnMut(Builder<'c, 's, Tok, S>) -> Builder<'c, 's, Tok, S>,
        Tok: TokenKind,
        S: Sink,
    {
        f: F,
        _phantom: PhantomData<(Tok, S)>,
    }

    impl<F, Tok, S> Parser<Tok, S> for ParseFn<F, Tok, S>
    where
        F: for<'c, 's> FnMut(Builder<'c, 's, Tok, S>) -> Builder<'c, 's, Tok, S>,
        Tok: TokenKind,
        S: Sink,
    {
        fn parse<'c, 's>(
            &mut self,
            state: State<'s, Tok, S>,
            ctx: Context<'c, Tok>,
        ) -> State<'s, Tok, S> {
            let builder = Builder { state, ctx };
            let Builder { state, .. } = (self.f)(builder);
            state
        }
    }

    pub fn parse<F, Tok, S>(f: F) -> ParseFn<F, Tok, S>
    where
        F: for<'c, 's> FnMut(Builder<'c, 's, Tok, S>) -> Builder<'c, 's, Tok, S>,
        Tok: TokenKind,
        S: Sink,
    {
        ParseFn {
            f,
            _phantom: PhantomData,
        }
    }

    // pub fn parse_once<'s, F, Tok, S>(f: F) -> impl Parser<'s, Tok, S>
    // where
    //     F: for<'c, 's> FnMut(Builder<'c, 's, Tok, S>) -> Builder<'c, 's, Tok, S>,
    //     Tok: TokenKind,
    //     S: Sink,
    // {
    //     ParseFn {
    //         f,
    //         _phantom: PhantomData,
    //     }
    // }
}

pub mod parsers {
    mod basic {
        use crate::{Parser, Context, Sink, TokenKind, parse};

        pub fn skip<Tok: TokenKind, S: Sink>() -> impl Parser<Tok, S> {
            parse(|s| s)
        }

        pub fn with_mode<Tok, Tok2, S>(mut parser: impl Parser<Tok2, S>) -> impl Parser<Tok, S>
        where
            S: Sink,
            Tok: TokenKind,
            Tok2: TokenKind,
            Tok::Extras: Into<Tok2::Extras>,
            Tok2::Extras: Into<Tok::Extras>,
        {
            parse(move |s| {
                let parser: &mut dyn Parser<Tok2, S> = &mut parser;
                s.with_mode(&mut *parser)
            })
        }

        pub fn with_ctx<'c, Tok, S>(ctx: Context<'c, Tok>, mut parser: impl Parser<Tok, S> + 'c) ->
        impl Parser<Tok, S> + 'c
        where
            S: Sink + 'c,
            Tok: TokenKind
        {
            parse(move |s| {
                let parser: &mut dyn Parser<Tok, S> = &mut parser;
                s.with_ctx(ctx, &mut *parser)
            })
        }
    }

    mod flow {
        use crate::{Parser, Peek, Sink, TokenKind, parse};

        pub fn repeated<Tok, S, F>(
            mut f: F,
            close: Tok
        ) -> impl Parser<Tok, S>
        where
            F: for<'c, 's> FnMut(Peek<'c, 's, Tok, S>) -> Peek<'c, 's, Tok, S>,
            Tok: TokenKind,
            S: Sink
        {
            parse(move |s| {
                match s.peek().at(close) {
                    Peek::Found { s, .. } => s,
                    Peek::None { mut s, .. } => loop {
                        s = match s.peek()
                            .at_unexpected(None)
                            .at(close) {
                            Peek::Found { s, .. } => break s,
                            p => (&mut f)(p).expect()
                        }
                    }
                }
            })
        }

        pub fn separated<Tok, S, P>(
            mut parser: P,
            separator: Tok,
            close: Tok,
            trailing: bool
        ) -> impl Parser<Tok, S>
        where
            P: Parser<Tok, S>,
            Tok: TokenKind + std::fmt::Debug,
            S: Sink
        {
            parse(move |s| match s.peek().at(close) {
                Peek::Found { s, .. } => s,
                Peek::None { mut s, .. } => 'outer: loop {
                    s = s.parse(&mut parser);
                    s = 'inner: loop {
                        let p = match s.peek()
                            .at_unexpected(None)
                            .at(close) {
                                Peek::Found { s, .. } => break 'outer s,
                                p => p
                            };
                        s = match p.at(separator) {
                            Peek::Found{ s, .. } =>
                            match s.token().peek().at(close) {
                                Peek::Found {s, ..} if trailing => break 'outer s,
                                p => break 'inner p.ignore_unexpected()
                            },
                            p => p.expect()
                        }
                    }
                }
            })
        }
    }
    mod infix {
        use crate::{Builder, Parser, Sink, TmpSink, TokenKind, parse};

        pub enum Assoc {
            Right,
            Left,
        }

        pub fn infix<Tok, S, L, BP, F>(left: L, bp: BP, f: F) -> impl Parser<Tok, S>
        where
            Tok: TokenKind,
            L: Parser<Tok, TmpSink>,
            S: Sink,
            BP: FnMut(Option<Tok>) -> Option<(Assoc, i32)>,
            F: for<'c, 's> FnMut(Builder<'c, 's, Tok, TmpSink>, &mut TmpSink, Option<Tok>)
                                 -> Builder<'c, 's, Tok, TmpSink>,
        {
            infix_inner(left, bp, f, 0)
        }

        fn infix_inner<Tok, S, L, BP, F>(mut left: L, mut bp: BP, mut f: F, rbp: i32) -> impl Parser<Tok, S>
        where
            Tok: TokenKind,
            L: Parser<Tok, TmpSink>,
            S: Sink,
            BP: FnMut(Option<Tok>) -> Option<(Assoc, i32)>,
            F: for<'c, 's> FnMut(Builder<'c, 's, Tok, TmpSink>,
                                 &mut TmpSink,
                                 Option<Tok>,) -> Builder<'c, 's, Tok, TmpSink>,
        {
            parse(move |mut s| {
                // Type erasure. Not pleasant but necessary
                let f: &mut dyn for<'c, 's>
                    FnMut(Builder<'c, 's, Tok, TmpSink>,
                          &mut TmpSink,
                          Option<Tok>,) -> Builder<'c, 's, Tok, TmpSink>
                    = &mut f;

                let left: &mut dyn Parser<Tok, TmpSink> = &mut left;

                let bp: &mut dyn
                    FnMut(Option<Tok>) -> Option<(Assoc, i32)>
                    = &mut bp;

                // -------
                let (s2, mut lhs) = s.with_sink(&mut *left);
                s = s2;

                loop {
                    let mut op_token = None;

                    let (s2, op_trivia): (_, TmpSink) = s.with_sink(parse(|mut s| {
                        op_token = Some(s.peek_token());
                        s
                    }));
                    s = s2;
                    lhs.append(op_trivia);

                    let op_token = op_token.unwrap(); // UGH

                    let (op_assoc, op_bp) = match (bp)(op_token.as_ref().copied()) {
                        Some(op) if op.1 > rbp => op,
                        _ => {
                            break s.insert(&mut lhs);
                        }
                    };


                    let new_op_bp = match op_assoc {
                        Assoc::Left => op_bp + 1,
                        Assoc::Right => op_bp - 1,
                    };

                    let (s2, new_lhs) : (_, TmpSink) = s.with_sink(parse(|s| {
                        (f)(s, &mut lhs, op_token)
                            .parse(
                                infix_inner(&mut *left, &mut *bp, &mut *f, new_op_bp - 1)
                            )
                            .end()
                    }));
                    lhs = new_lhs;
                    s = s2;
                }
            })
        }
    }

    pub use basic::*;
    pub use flow::*;
    pub use infix::*;
}

pub use smol_str::SmolStr;
pub use text_size::{TextLen, TextRange, TextSize};
pub use microtree_derive::TokenKind;

pub use error::*;
pub use event::*;
pub use name::*;
pub use sink::*;

pub use sinks::*;

pub use source::*;
pub use lexer::*;
pub use callback_result::*;
pub use peekable::*;
pub use spanned::*;
pub use token_kind::*;

pub use peek::*;
pub use builder::*;
pub use context::*;
pub use parse_fn::*;
pub use parser::*;
pub use state::*;
pub use trivia_fn::*;

#[cfg(test)]
mod tests {
    use crate::parsers::{Assoc, separated, with_ctx};

    use super::*;
    use parsers::infix;

    fn mergeable(token: Token, other: Token) -> bool {
        token == Token::Error && other == token
    }

    fn lex_number(_bumped: TextSize, _source: &mut Source<'_>, _extras: &mut String) -> bool {
        //*extras = "foo".to_string();
        //eprintln!("foo: {}", extras);
        //Some(bumped)
        true
    }

    #[derive(Debug, PartialEq, Clone, Copy, TokenKind)]
    #[token_kind(extras = "String", mergeable = "mergeable")]
    enum Token {
        #[token_kind(regex = r"[0-9]+", callback="lex_number")]
        Number,

        #[token_kind(token = "(")]
        OpenP,

        #[token_kind(token = ")")]
        CloseP,

        #[token_kind(token = "[")]
        OpenB,

        #[token_kind(token = "]")]
        CloseB,

        #[token_kind(token = "{", display = "`{{`")]
        OpenC,

        #[token_kind(token = "}", display = "`}}`")]
        CloseC,

        #[token_kind(token = ",")]
        Comma,

        #[token_kind(token = "-")]
        OpMinus,

        #[token_kind(token = "+")]
        OpPlus,

        #[token_kind(token = "*")]
        OpStar,

        #[token_kind(token = "/")]
        OpSlash,

        #[token_kind(regex = r"(\r?\n)+", display = "line ending")]
        LineEnd,

        #[token_kind(regex = r"[ \t]+", display = "space or tab")]
        Whitespace,

        #[token_kind(error)]
        Error,
    }

    struct Nodes;
    #[allow(non_upper_case_globals)]
    impl Nodes {
        pub const Number: Name = Name::new("number");
        pub const Value: Name = Name::new("Value");
        pub const Unary: Name = Name::new("Unary");
        pub const Binary: Name = Name::new("Binary");
        pub const Op: Name = Name::new("Op");
        pub const Array: Name = Name::new("Array");
    }

    type Lexer<'s, T = Token> = crate::Lexer<'s, T>;

    fn left_value<S: Sink>() -> impl Parser<Token, S> {
        parse(|s| {
            s.peek()
             .at(Token::OpenP).parse(parens())
             .at(Token::OpenC).parse(mode_test())
             .parse_else(parse(|s| s.alias(Nodes::Value)))
             .at(Token::Number).parse(number())
             .at(Token::OpMinus).parse(unary())
             .at(Token::OpenB).parse(array())
             .expect()
        })
    }

    fn parens<S: Sink>() -> impl Parser<Token, S> {
        parse(|s| s
              .token()
              .parse(value())
              .expect(Token::CloseP))
    }

    fn mode_test<S: Sink>() -> impl Parser<Token, S> {
        let leading = leading();
        let trailing = trailing();
        parse(move |s| s
              .token()
              .with_mode(with_ctx(Context {
                    leading: Some(&leading),
                    trailing: Some(&trailing),
              }, value()))
              .expect(Token::CloseC))
    }

    fn array<S: Sink>() -> impl Parser<Token, S> {
        parse(|s| {
                s.start(Nodes::Array)
                .token()
                .parse(separated(value(), Token::Comma, Token::CloseB, true))
                .expect(Token::CloseB)
                .end()
        })
    }

    fn number<S: Sink>() -> impl Parser<Token, S> {
        parse(|s| s.alias(Nodes::Number).token())
    }

    fn unary<S: Sink>() -> impl Parser<Token, S> {
        parse(|s| {
            s.start(Nodes::Unary)
                .expect(Token::OpMinus)
                .parse(value())
                .end()
        })
    }

    fn value<S: Sink>() -> impl Parser<Token, S> {
        infix(
            left_value(),
            |token| match token {
                Some(Token::OpStar) => Some((Assoc::Left, 20)),
                Some(Token::OpSlash) => Some((Assoc::Left, 20)),

                Some(Token::OpMinus) => Some((Assoc::Left, 10)),
                Some(Token::OpPlus) => Some((Assoc::Left, 10)),

                _ => None,
            },
            |s, left, _op_token| {
                s.alias(Nodes::Value)
                    .start(Nodes::Binary)
                    .insert(left)
                    .alias(Nodes::Op)
                    .token()
            },
        )
    }

    fn leading() -> impl Trivia<Token> {
        trivia(|s| {
            while let Some(Token::LineEnd) = s.peek_token() {
                s.next();
            }
            while let Some(Token::LineEnd) | Some(Token::Whitespace) = s.peek_token() {
                s.next();
            }
        })
    }

    fn trailing() -> impl Trivia<Token> {
        trivia(|s| {
            while let Some(Token::Whitespace) = s.peek_token() {
                s.next();
            }
        })
    }

    fn root<S: Sink>() -> impl Parser<Token, S> {
        parse(|s| {
            let leading = leading();
            let trailing = trailing();
            s.with_ctx(
                Context {
                    leading: Some(&leading),
                    trailing: Some(&trailing),
                },
                value()
            )
            .expect(None)
        })
    }

    fn assert_ok(input: &str, expected_events: &str, expected_tree: &str) {
        use diff_assert::assert_diff;
        let lexer = Lexer::new(input);
        let sink: TestSink = State::parse(lexer, root());

        let mut events = sink.events.join("\n");
        if !sink.errors.is_empty() {
            let errors = sink.errors.join("\n");
            panic!("Found errors: \n{}", errors);
        }

        let mut expected_events = String::from(expected_events.trim());
        events.retain(|c| c != ' ' && c != '\t');
        expected_events.retain(|c| c != ' ' && c != '\t');

        assert_diff!(expected_events.replace(";", "\n"), events);

        let lexer = Lexer::new(input);
        let sink: GreenSink = State::parse(lexer, root());

        let tree = sink.finish().root;
        let tree = format!("{:?}", tree);
        let expected_tree = unindent::unindent(expected_tree);

        assert_diff!(expected_tree, tree);
    }

    #[test]
    fn simple() {
        assert_ok(
            "2+3",
            r#"
            Alias("Value"); Start("Binary")
                Alias("Value"); Alias("number"); Token("token", "2")
                Alias("Op"); Token("token", "+")
                Alias("Value"); Alias("number"); Token("token", "3")
            End
            Eof
            "#,
            r#"
            --- GREEN TREE ---
            Root, Value, Binary @ 0..3
                Value, number, token @ 0..1 = `2`
                Op, token @ 1..2 = `+`
                Value, number, token @ 2..3 = `3`
            --- END ---
            "#,
        );
    }

    #[test]
    fn complex() {
        assert_ok(
            "2+3*4",
            r#"
            Alias("Value"); Start("Binary")
                Alias("Value"); Alias("number"); Token("token", "2")
                Alias("Op"); Token("token", "+")
                Alias("Value")
                Start("Binary")
                    Alias("Value"); Alias("number"); Token("token", "3")
                    Alias("Op"); Token("token", "*")
                    Alias("Value"); Alias("number"); Token("token", "4")
                End
            End
            Eof
            "#,
            r#"
            --- GREEN TREE ---
            Root, Value, Binary @ 0..5
                Value, number, token @ 0..1 = `2`
                Op, token @ 1..2 = `+`
                Value, Binary @ 2..5
                    Value, number, token @ 2..3 = `3`
                    Op, token @ 3..4 = `*`
                    Value, number, token @ 4..5 = `4`
            --- END ---
            "#,
        );
    }

    #[test]
    fn complex_different() {
        assert_ok(
            "2+3+4",
            r#"
            Alias("Value"); Start("Binary")
                Alias("Value"); Start("Binary")
                    Alias("Value"); Alias("number"); Token("token", "2")
                    Alias("Op"); Token("token", "+")
                    Alias("Value"); Alias("number"); Token("token", "3")
                End
                Alias("Op"); Token("token", "+")
                Alias("Value"); Alias("number"); Token("token", "4")
            End
            Eof
            "#,
            r#"
            --- GREEN TREE ---
            Root, Value, Binary @ 0..5
                Value, Binary @ 0..3
                    Value, number, token @ 0..1 = `2`
                    Op, token @ 1..2 = `+`
                    Value, number, token @ 2..3 = `3`
                Op, token @ 3..4 = `+`
                Value, number, token @ 4..5 = `4`
            --- END ---
            "#,
        );
    }

    #[test]
    fn test_parens() {
        assert_ok(
            "(2+3)*4",
            r#"
            Alias("Value"); Start("Binary")
                Token("token", "(")
                Alias("Value"); Start("Binary")
                    Alias("Value"); Alias("number"); Token("token", "2")
                    Alias("Op"); Token("token", "+")
                    Alias("Value"); Alias("number"); Token("token", "3")
                End
                Token("token", ")")
                Alias("Op"); Token("token", "*")
                Alias("Value"); Alias("number"); Token("token", "4")
            End
            Eof
            "#,
            r#"
            --- GREEN TREE ---
            Root, Value, Binary @ 0..7
                token @ 0..1 = `(`
                Value, Binary @ 1..4
                    Value, number, token @ 1..2 = `2`
                    Op, token @ 2..3 = `+`
                    Value, number, token @ 3..4 = `3`
                token @ 4..5 = `)`
                Op, token @ 5..6 = `*`
                Value, number, token @ 6..7 = `4`
            --- END ---
            "#,
        );
    }

    #[test]
    fn leading_trivia() {
        assert_ok(
            "2+\n 3",
            r#"
            Alias("Value"); Start("Binary")
                Alias("Value"); Alias("number"); Token("token", "2")
                Alias("Op"); Token("token", "+")
                Trivia(Leading, "\n ")
                Alias("Value"); Alias("number"); Token("token", "3")
            End
            Eof
            "#,
            r#"
            --- GREEN TREE ---
            Root, Value, Binary @ 0..5
                Value, number, token @ 0..1 = `2`
                Op, token @ 1..2 = `+`
                Value, number, token @ 2..5 = `3` ; leading: `\n `
            --- END ---
            "#,
        );
    }

    #[test]
    fn leading_trivia_2() {
        assert_ok(
            "2\n +3",
            r#"
            Alias("Value"); Start("Binary")
                Alias("Value"); Alias("number"); Token("token", "2")
                Trivia(Leading, "\n ")
                Alias("Op"); Token("token", "+")
                Alias("Value"); Alias("number"); Token("token", "3")
            End
            Eof
            "#,
            r#"
            --- GREEN TREE ---
            Root, Value, Binary @ 0..5
                Value, number, token @ 0..1 = `2`
                Op, token @ 1..4 = `+` ; leading: `\n `
                Value, number, token @ 4..5 = `3`
            --- END ---
            "#,
        );
    }

    #[test]
    fn trailing_trivia() {
        assert_ok(
            "2+3 ",
            r#"
            Alias("Value"); Start("Binary")
                Alias("Value"); Alias("number"); Token("token", "2")
                Alias("Op"); Token("token", "+")
                Alias("Value"); Alias("number"); Trivia(Trailing, " "); Token("token", "3")
            End
            Eof
            "#,
            r#"
            --- GREEN TREE ---
            Root, Value, Binary @ 0..4
                Value, number, token @ 0..1 = `2`
                Op, token @ 1..2 = `+`
                Value, number, token @ 2..4 = `3` ; trailing: ` `
            --- END ---
            "#,
        );
    }

    #[test]
    fn array_tests() {
        assert_ok(
            "[ 2+3, 4]",
            r#"
            Alias("Value"); Start("Array")
                Trivia(Trailing, " "); Token("token", "[")
                Alias("Value"); Start("Binary")
                    Alias("Value"); Alias("number"); Token("token", "2")
                    Alias("Op"); Token("token", "+")
                    Alias("Value"); Alias("number"); Token("token", "3")
                End
                Trivia(Trailing, " "); Token("token", ",")
                Alias("Value"); Alias("number"); Token("token", "4")
                Token("token", "]")
            End
            Eof
            "#,
            r#"
            --- GREEN TREE ---
            Root, Value, Array @ 0..9
                token @ 0..2 = `[` ; trailing: ` `
                Value, Binary @ 2..5
                    Value, number, token @ 2..3 = `2`
                    Op, token @ 3..4 = `+`
                    Value, number, token @ 4..5 = `3`
                token @ 5..7 = `,` ; trailing: ` `
                Value, number, token @ 7..8 = `4`
                token @ 8..9 = `]`
            --- END ---
            "#,
        );
        assert_ok(
            // Input
            "[ 2+3, 4,]",
            // Expected events
            r#"
            Alias("Value"); Start("Array")
                Trivia(Trailing, " "); Token("token", "[")
                Alias("Value"); Start("Binary")
                    Alias("Value"); Alias("number"); Token("token", "2")
                    Alias("Op"); Token("token", "+")
                    Alias("Value"); Alias("number"); Token("token", "3")
                End
                Trivia(Trailing, " "); Token("token", ",")
                Alias("Value"); Alias("number"); Token("token", "4")
                Token("token", ",")
                Token("token", "]")
            End
            Eof
            "#,
            // Expected tree
            r#"
            --- GREEN TREE ---
            Root, Value, Array @ 0..10
                token @ 0..2 = `[` ; trailing: ` `
                Value, Binary @ 2..5
                    Value, number, token @ 2..3 = `2`
                    Op, token @ 3..4 = `+`
                    Value, number, token @ 4..5 = `3`
                token @ 5..7 = `,` ; trailing: ` `
                Value, number, token @ 7..8 = `4`
                token @ 8..9 = `,`
                token @ 9..10 = `]`
            --- END ---
            "#,
        );
    }

    #[test]
    fn not_infix_trailing() {
        assert_ok(
            "2\n",
            r#"
            Alias("Value"); Alias("number"); Token("token", "2")
            Trivia(Leading, "\n")
            Eof
            "#,
            r#"
            --- GREEN TREE ---
            Root @ 0..2
                Value, number, token @ 0..1 = `2`
                eof @ 1..2 = `` ; leading: `\n`
            --- END ---
            "#,
        );
    }


    #[test]
    fn mode_tests() {
        assert_ok(
            "{2\n}",
            r#"
            Token("token", "{")
            Alias("Value"); Alias("number"); Token("token", "2")
            Trivia(Leading, "\n")
            IgnoreTrivia
            Trivia(Leading, "\n")
            Token("token", "}")
            Eof
            "#,
            r#"
            --- GREEN TREE ---
            Root @ 0..4
                token @ 0..1 = `{`
                Value, number, token @ 1..2 = `2`
                token @ 2..4 = `}` ; leading: `\n`
            --- END ---
            "#,
        );
    }


}
