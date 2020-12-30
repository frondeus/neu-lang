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
    #[derive(Debug)]
    pub enum Event {
        Start(Name),
        Alias(Name),
        Token(Name, SmolStr),
        Unfinished,
        Abort,
        Preceeds,
        Finish(Name),
        End,
        Trivia(TriviaKind, SmolStr),
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
            pre: Vec<Green>,
            roots: Vec<Green>,
            errors: Vec<Error>,
            leading: Option<SmolStr>,
            trailing: Option<SmolStr>,
        }

        impl GreenSink {
            pub fn finish(mut self) -> ParseResult {
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

            fn add(&mut self, green: Green, preceeds: bool) {
                if !preceeds {
                    match self.current() {
                        Some(parent) => parent.children.push(green),
                        None => self.roots.push(green),
                    }
                }
                else {
                    self.pre.push(green);
                }
            }

            fn add_many(&mut self, mut green: Vec<Green>, preceeds: bool) {
                if !preceeds {
                    match self.current() {
                        Some(parent) => parent.children.append(&mut green),
                        None => self.roots.append(&mut green),
                    }
                } else {
                    self.pre.append(&mut green);
                }
            }

            fn pre(&mut self) -> Vec<Green> {
                std::mem::take(&mut self.pre)
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
                            let children = self.pre();
                            self.stack.push(UnsealedGreen {
                                names: vec![name],
                                children,
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
                            let children = self.pre();
                            self.stack.push(UnsealedGreen {
                                names: vec![],
                                children,
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
                            let children = self.pre();
                            self.stack.push(UnsealedGreen {
                                names: vec![name],
                                aliased: true,
                                children,
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
                        self.add_many(children, false);
                    }
                    Event::Finish(name) => {
                        let current = self.stack.pop().expect("Unmatched End");
                        let aliases = current.names.into_iter().rev();

                        let mut green = self.cache.node(name.into(), current.children);
                        for alias in aliases {
                            green = self.cache.alias(alias.into(), green);
                        }

                        self.add(green, false);
                    }
                    Event::Preceeds => {
                        let current = self.stack.pop().expect("Unmatched End");
                        if current.unfinished {
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
                            self.add_many(children, true);
                        } else {
                            let mut names = current.names.into_iter().rev();
                            let name = names.next().unwrap_or_default();
                            let aliases = names;

                            let mut green = self.cache.node(name.into(), current.children);
                            for alias in aliases {
                                green = self.cache.alias(alias.into(), green);
                            }

                            self.add(green, true);
                        }
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

                        self.add(green, false);
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
                    Event::Trivia(TriviaKind::Leading, value) => {
                        self.leading.get_or_insert(value);
                    }
                    Event::Trivia(TriviaKind::Trailing, value) => {
                        self.trailing.get_or_insert(value);
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

            #[test]
            fn precedence() {
                let mut sink = GreenSink::default();
                sink.event(Event::alias("Value"));
                sink.event(Event::Unfinished);
                sink.event(Event::token("number", "2"));
                sink.event(Event::Preceeds);
                sink.event(Event::start("binary"));
                sink.event(Event::token("op", "+"));
                sink.event(Event::alias("Value"));
                sink.event(Event::token("number", "2"));
                sink.event(Event::End);

                test_event(
                    sink,
                    r#"--- GREEN TREE ---
                                    Root, binary @ 0..3
                                        Value, number @ 0..1 = `2`
                                        op @ 1..2 = `+`
                                        Value, number @ 2..3 = `2`
                                    --- END ---"#,
                );
            }

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

    pub use wrapper_sink::*;
    pub use green_sink::*;
    pub use test_sink::*;
    pub use write_sink::*;
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
    use logos::Logos;

    pub trait TokenKind<'s>:
        Logos<'s, Source = str, Extras: Clone>
        + std::fmt::Display
        + PartialEq
        + Clone
        + Copy
        + Send
        + Sync
    {
        fn mergeable(self, _other: Self) -> bool {
            false
        }
    }
}

mod lexer {
    use crate::{PeekableIterator, SmolStr, Spanned, TextRange, TextSize, TokenKind};
    use logos::Lexer as Inner;

    #[derive(Clone)]
    pub struct Lexer<'s, Tok: TokenKind<'s>> {
        #[allow(clippy::option_option)]
        peeked: Option<Option<(Inner<'s, Tok>, Spanned<Tok>)>>,
        inner: Inner<'s, Tok>,
    }

    impl<'s, Tok: TokenKind<'s>> Lexer<'s, Tok> {
        pub fn new(source: &'s Tok::Source) -> Self {
            Self {
                inner: Inner::new(source),
                peeked: None,
            }
        }

        pub fn morph<Tok2>(self) -> Lexer<'s, Tok2>
        where
            Tok2: TokenKind<'s>,
            Tok::Extras: Into<Tok2::Extras>,
        {
            Lexer {
                peeked: None,
                inner: self.inner.morph(),
            }
        }

        pub fn span(&self) -> TextRange {
            let range = self.inner.span();
            TextRange::new((range.start as u32).into(), (range.end as u32).into())
        }

        pub(crate) fn text_for_span(&self, span: TextRange) -> SmolStr {
            let source = self.inner.source();
            source[span].into()
        }

        pub fn peek_token(&mut self) -> Option<Tok> {
            self.peek().map(|t| t.token)
        }

        pub(crate) fn inner(&self) -> &Inner<'s, Tok> {
            &self.inner
        }
    }

    impl<'s, Tok> PeekableIterator for Lexer<'s, Tok>
    where
        Tok: TokenKind<'s>,
    {
        fn peek(&mut self) -> Option<&Self::Item> {
            if self.peeked.is_none() {
                let saved = self.inner.clone();
                let token = self.next();
                let original = std::mem::replace(&mut self.inner, saved);
                self.peeked = Some(token.map(|token| (original, token)));
            }

            self.peeked
                .as_ref()
                .and_then(|t| t.as_ref())
                .map(|(_, t)| t)
        }
    }

    impl<'s, Tok: TokenKind<'s>> Iterator for Lexer<'s, Tok> {
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
                        let new_value = &self.inner.source()[range];
                        first.value = new_value.into();
                        self.lex();
                    }
                    _ => break Some(first),
                }
            }
        }
    }

    impl<'s, Tok: TokenKind<'s>> Lexer<'s, Tok> {
        fn lex(&mut self) -> Option<Spanned<Tok>> {
            if let Some(peeked) = self.peeked.take() {
                if let Some((original, peeked)) = peeked {
                    self.inner = original;
                    return Some(peeked);
                }
                return None;
            }
            let token = self.inner.next()?;
            let value = self.inner.slice().into();
            let range = self.span();
            Some(Spanned {
                token,
                range,
                value,
            })
        }

        fn peek_one(&mut self) -> Option<&Spanned<Tok>> {
            if self.peeked.is_none() {
                let saved = self.inner.clone();
                let token = self.lex();
                let original = std::mem::replace(&mut self.inner, saved);
                self.peeked = Some(token.map(|token| (original, token)));
            }

            self.peeked
                .as_ref()
                .and_then(|t| t.as_ref())
                .map(|(_, t)| t)
        }
    }
}

mod parser {
    use crate::{Context, Sink, State, TokenKind};

    pub trait Parser<Tok, S>
    where
        Tok: for<'s> TokenKind<'s>,
        S: Sink,
    {
        fn parse<'s, 'c>(
            &mut self,
            state: State<'s, Tok, S>,
            context: Context<'c, Tok>,
        ) -> State<'s, Tok, S>;
    }
}

mod context {
    use crate::{Lexer, TokenKind};

    pub trait Trivia<Tok>
    where
        Tok: for<'s> TokenKind<'s>,
    {
        fn parse<'s>(&self, state: &mut Lexer<'s, Tok>);
    }

    pub struct Context<'c, Tok>
    where
        Tok: for<'s> TokenKind<'s>,
    {
        pub leading: Option<&'c dyn Trivia<Tok>>,
        pub trailing: Option<&'c dyn Trivia<Tok>>,
    }

    impl<'c, Tok> Default for Context<'c, Tok>
    where
        Tok: for<'s> TokenKind<'s> //+ 'c,
    {
        fn default() -> Self {
            Self {
                leading: None,
                trailing: None,
            }
        }
    }

    impl<'c, Tok> Clone for Context<'c, Tok> where Tok: for<'s> TokenKind<'s> {
        fn clone(&self) -> Self {
            Self {
                leading: self.leading.clone(),
                trailing: self.trailing.clone()
            }
        }
    }
    impl<'c, Tok> Copy for Context<'c, Tok> where Tok: for<'s> TokenKind<'s> {}

    impl<'c, Tok> Context<'c, Tok>
    where
        Tok: for<'s> TokenKind<'s> //+ 'c,
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
        Tok: TokenKind<'s>,
        S: Sink,
    {
        pub(crate) lexer: Lexer<'s, Tok>,
        pub(crate) sink: S,
    }

    impl<'s, Tok, S> State<'s, Tok, S>
    where
        Tok: for<'s2> TokenKind<'s2>,
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
        Tok: TokenKind<'s>,
        S: Sink,
    {
        pub fn new(
            lexer: Lexer<'s, Tok>,
            sink: S,
        ) -> Self {
            Self {
                sink,
                lexer,
            }
        }

        pub fn morph<Tok2>(self) -> State<'s, Tok2, S>
        where Tok2: TokenKind<'s>,
              Tok::Extras: Into<Tok2::Extras>
        {
            let Self { lexer, sink } = self;
            State {
                sink,
                lexer: lexer.morph()
            }
        }

        pub fn sink_mut(&mut self) -> &mut S {
            &mut self.sink
        }
        pub fn lexer_mut(&mut self) -> &mut Lexer<'s, Tok> {
            &mut self.lexer
        }
    }
}

mod builder {
    use itertools::Itertools;

    use crate::{
        Context, Event, Lexer, Name, Parser, Sink, Spanned, State, TextRange, TokenKind, Trivia,
        TriviaKind,
    };

    use logos::Logos;

    pub enum Peek<'c, 's, Tok, S>
    where
        Tok: for<'s2> TokenKind<'s2>,
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
        Tok: for<'s2> TokenKind<'s2> + std::fmt::Debug,
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
        Tok: for<'s2> TokenKind<'s2>,
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
                    s.state
                        .sink_mut()
                        .event(Event::token("error", err_value.clone()));
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

    pub struct Builder<'c, 's, Tok, S>
    where
        Tok: for<'s2> TokenKind<'s2>,
        S: Sink,
    {
        pub(crate) state: State<'s, Tok, S>,
        pub(crate) ctx: Context<'c, Tok>,
    }

    impl<'c, 's, Tok, S> Builder<'c, 's, Tok, S>
    where
        Tok: for<'s2> TokenKind<'s2>,
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
                let start = self.state.lexer_mut().span().end();
                trivia.parse(self.state.lexer_mut());
                let end = self.state.lexer_mut().span().end();
                if start != end {
                    let text_range = TextRange::new(start, end);
                    let value = self.state.lexer_mut().text_for_span(text_range);
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

        pub fn token(mut self) -> Self {
            if let Some(value) = self.next_token().map(|t| t.value) {
                self.state.sink_mut().event(Event::token("token", value));
            }
            self
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
                if let Some(value) = next.map(|t| t.value) {
                    self.state.sink_mut().event(Event::token("token", value));
                }
                return self;
            }
            let err_value = next
                .as_ref()
                .map(|e| e.value.to_string())
                .unwrap_or_else(|| "".to_string());
            self.state
                .sink_mut()
                .event(Event::token("error", err_value.clone()));
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

        pub fn preceeds(mut self) -> Self {
            self.state.sink_mut().event(Event::Preceeds);
            self
        }

        pub fn with_ctx<'c2>(self, ctx2: Context<'c2, Tok>, mut parser: impl Parser<Tok, S>) -> Self {
            let Self { state, ctx } = self;

            let state  = parser.parse(state, ctx2);

            Self { state, ctx }
        }

        pub fn with_mode<'c2, Tok2>(self, mut parser: impl Parser<Tok2, S>) -> Self
        where
            Tok2: for<'s2> TokenKind<'s2>,
            for<'s2> <Tok as Logos<'s2>>::Extras: Into<<Tok2 as Logos<'s2>>::Extras>,
            for<'s2> <Tok2 as Logos<'s2>>::Extras: Into<<Tok as Logos<'s2>>::Extras>,
        {
            let Self { state, ctx } = self;

            let inner = Context::default();

            let state = parser.parse(state.morph(), inner).morph();

            Self { state, ctx }
        }

        pub fn with_range(self, range: TextRange, mut parser: impl Parser<Tok, S>) -> Self {
            let Self { mut state, ctx } = self;

            let source = state.lexer_mut().inner().source();
            let subsource = &source[range];
            let sublexer = Lexer::new(subsource);

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
        Tok: for<'s> TokenKind<'s>,
    {
        f: F,
        _phantom: PhantomData<Tok>,
    }

    impl<F, Tok> Clone for TriviaFn<F, Tok>
    where
        F: Clone + for<'s> Fn(&mut Lexer<'s, Tok>),
        Tok: for<'s> TokenKind<'s>,
    {
        fn clone(&self) -> Self {
            Self {
                f: self.f.clone(),
                _phantom: PhantomData,
            }
        }
    }

    impl<F, Tok> Trivia<Tok> for TriviaFn<F, Tok>
    where
        F: for<'s> Fn(&mut Lexer<'s, Tok>),
        Tok: for<'s> TokenKind<'s>,
    {
        fn parse<'s>(&self, state: &mut Lexer<'s, Tok>) {
            (self.f)(state)
        }
    }

    pub fn trivia<F, Tok>(f: F) -> impl Trivia<Tok> + Clone
    where
        F: Clone + for<'s> Fn(&mut Lexer<'s, Tok>),
        Tok: for<'s> TokenKind<'s>,
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
        Tok: for<'s> TokenKind<'s>,
        S: Sink,
    {
        f: F,
        _phantom: PhantomData<(Tok, S)>,
    }

    impl<F, Tok, S> Clone for ParseFn<F, Tok, S>
    where
        F: Clone + for<'c, 's> FnMut(Builder<'c, 's, Tok, S>) -> Builder<'c, 's, Tok, S>,
        Tok: for<'s> TokenKind<'s>,
        S: Sink,
    {
        fn clone(&self) -> Self {
            Self {
                f: self.f.clone(),
                _phantom: PhantomData,
            }
        }
    }

    impl<F, Tok, S> Parser<Tok, S> for ParseFn<F, Tok, S>
    where
        F: for<'c, 's> FnMut(Builder<'c, 's, Tok, S>) -> Builder<'c, 's, Tok, S>,
        Tok: for<'s> TokenKind<'s>,
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
        F: Clone + for<'c, 's> FnMut(Builder<'c, 's, Tok, S>) -> Builder<'c, 's, Tok, S>,
        Tok: for<'s> TokenKind<'s>,
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
    //     Tok: TokenKind<'s>,
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
        use logos::Logos;

        pub fn skip<Tok: for<'s> TokenKind<'s>, S: Sink>() -> impl Parser<Tok, S> {
            parse(|s| s)
        }

        pub fn with_mode<Tok, Tok2, S>(parser: impl Parser<Tok2, S> + Clone) -> impl Parser<Tok, S> + Clone
        where
            S: Sink,
            Tok: for<'s> TokenKind<'s>,
            Tok2: for<'s> TokenKind<'s>,
            for<'s> <Tok as Logos<'s>>::Extras: Into<<Tok2 as Logos<'s>>::Extras>,
            for<'s> <Tok2 as Logos<'s>>::Extras: Into<<Tok as Logos<'s>>::Extras>,
        {
            parse(move |s| s.with_mode(parser.clone()))
        }

        pub fn with_ctx<'c, Tok, S>(ctx: Context<'c, Tok>, parser: impl Parser<Tok, S> + Clone + 'c) ->
        impl Parser<Tok, S> + 'c + Clone
        where
            S: Sink + 'c,
            Tok: for<'s> TokenKind<'s>
        {
            parse(move |s| s.with_ctx(ctx, parser.clone()))
        }
    }

    mod flow {
        use crate::{Parser, Peek, Sink, TokenKind, parse};

        pub fn repeated<Tok, S, F>(
            f: F,
            close: Tok
        ) -> impl Parser<Tok, S>
        where
            F: Clone + for<'c, 's> FnMut(Peek<'c, 's, Tok, S>) -> Peek<'c, 's, Tok, S>,
            Tok: for<'s> TokenKind<'s>,
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
                            p => (f.clone())(p).expect()
                        }
                    }
                }
            })
        }

        pub fn separated<Tok, S, P>(
            parser: P,
            separator: Tok,
            close: Tok,
            trailing: bool
        ) -> impl Parser<Tok, S>
        where
            P: Parser<Tok, S> + Clone,
            Tok: for<'s> TokenKind<'s> + std::fmt::Debug,
            S: Sink
        {
            parse(move |s| match s.peek().at(close) {
                Peek::Found { s, .. } => s,
                Peek::None { mut s, .. } => 'outer: loop {
                    s = s.parse(parser.clone());
                    s = 'inner: loop {
                        let p = match s.peek()
                            .at_unexpected(None)
                            .at(close) {
                                Peek::Found { s, .. } => break 'outer s,
                                p => p
                            };
                        s = match p.at(separator) {
                            Peek::Found{ s, .. } =>
                            match dbg!(s.token().peek()).at(close) {
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
    mod pratt {
        use crate::{Builder, Parser, Sink, TokenKind, parse};

        pub enum Assoc {
            Right,
            Left,
        }

        pub fn pratt<Tok, L, S, BP, F>(left: L, bp: BP, f: F) -> impl Parser<Tok, S> + Clone
        where
            Tok: for<'s> TokenKind<'s>,
            L: Clone + Parser<Tok, S>,
            S: Sink,
            BP: Clone + FnMut(Option<Tok>) -> Option<(Assoc, i32)>,
            F: Clone + for<'c, 's> FnMut(Builder<'c, 's, Tok, S>, Option<Tok>) -> Builder<'c, 's, Tok, S>,
        {
            pratt_inner(left, bp, f, 0)
        }

        fn pratt_inner<Tok, L, S, BP, F>(left: L, mut bp: BP, mut f: F, rbp: i32) -> impl Parser<Tok, S> + Clone
        where
            Tok: for<'s> TokenKind<'s>,
            L: Clone + Parser<Tok, S>,
            S: Sink,
            BP: Clone + FnMut(Option<Tok>) -> Option<(Assoc, i32)>,
            F: Clone + for<'c, 's> FnMut(Builder<'c, 's, Tok, S>, Option<Tok>,) -> Builder<'c, 's, Tok, S>,
        {
            parse(move |mut s| {
                s = s.unfinished().parse(left.clone());

                let mut first = true;
                loop {
                    let op_token = s.peek_token();
                    let (op_assoc, op_bp) = match (bp)(op_token.as_ref().copied()) {
                        Some(op) if op.1 > rbp => op,
                        _ if first => {
                            break s.abort();
                        }
                        _ => {
                            break s.end();
                        }
                    };

                    first = false;
                    let new_op_bp = match op_assoc {
                        Assoc::Left => op_bp + 1,
                        Assoc::Right => op_bp - 1,
                    };

                    s = s.preceeds();

                    s = (f)(s, op_token)
                        .parse(
                            pratt_inner(left.clone(), bp.clone(), f.clone(), new_op_bp - 1)
                        );
                }
            })
        }
    }

    pub use basic::*;
    pub use flow::*;
    pub use pratt::*;
}

pub use smol_str::SmolStr;
pub use text_size::{TextLen, TextRange, TextSize};

pub use error::*;
pub use event::*;
pub use name::*;
pub use sink::*;

pub use sinks::*;

pub use lexer::*;
pub use peekable::*;
pub use spanned::*;
pub use token_kind::*;

pub use builder::*;
pub use context::*;
pub use parse_fn::*;
pub use parser::*;
pub use state::*;
pub use trivia_fn::*;

#[cfg(test)]
mod tests {
    use crate::parsers::{Assoc, separated};

    use super::*;
    use derive_more::Display;
    use logos::Logos;
    use parsers::pratt;

    #[derive(Logos, Debug, PartialEq, Clone, Copy, Display)]
    enum Token {
        #[display(fmt = "number")]
        #[regex("[0-9]+")]
        Number,

        #[display(fmt = "`(`")]
        #[token("(")]
        OpenP,

        #[display(fmt = "`)`")]
        #[token(")")]
        CloseP,

        #[display(fmt = "`[`")]
        #[token("[")]
        OpenB,

        #[display(fmt = "`]`")]
        #[token("]")]
        CloseB,

        #[display(fmt = "`,`")]
        #[token(",")]
        Comma,

        #[display(fmt = "`-`")]
        #[token("-")]
        OpMinus,

        #[display(fmt = "`+`")]
        #[token("+")]
        OpPlus,

        #[display(fmt = "`*`")]
        #[token("*")]
        OpStar,

        #[display(fmt = "`/`")]
        #[token("/")]
        OpSlash,

        #[display(fmt = "line ending")]
        #[regex(r"(\r?\n)+")]
        LineEnd,

        #[display(fmt = "space or tab")]
        #[regex("[ \t]+")]
        Whitespace,

        #[display(fmt = "error")]
        #[error]
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

    impl TokenKind<'_> for Token {
        fn mergeable(self, other: Self) -> bool {
            self == Self::Error && self == other
        }
    }

    type Lexer<'s, T = Token> = crate::Lexer<'s, T>;

    fn left_value<S: Sink>() -> impl Parser<Token, S> + Clone {
        parse(|s| {
            s.peek()
             .at(Token::OpenP).parse(parse(|s| s.token().parse(value()).expect(Token::CloseP)))
             .parse_else(parse(|s| s.alias(Nodes::Value)))
             .at(Token::Number).parse(number())
             .at(Token::OpMinus).parse(unary())
             .at(Token::OpenB).parse(array())
             .expect()
        })
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

    fn value<S: Sink>() -> impl Parser<Token, S> + Clone {
        pratt(
            left_value(),
            |token| match token {
                Some(Token::OpStar) => Some((Assoc::Left, 20)),
                Some(Token::OpSlash) => Some((Assoc::Left, 20)),

                Some(Token::OpMinus) => Some((Assoc::Left, 10)),
                Some(Token::OpPlus) => Some((Assoc::Left, 10)),

                _ => None,
            },
            |s, _op_token| {
                s.alias(Nodes::Value)
                    .start(Nodes::Binary)
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
            Unfinished
                Alias("Value"); Alias("number"); Token("token", "2")
            Preceeds
            Alias("Value"); Start("Binary")
                Alias("Op"); Token("token", "+")
                Unfinished
                    Alias("Value"); Alias("number"); Token("token", "3")
                Abort
            End
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
            Unfinished
                Alias("Value"); Alias("number"); Token("token", "2")
            Preceeds
            Alias("Value"); Start("Binary")
                Alias("Op"); Token("token", "+")
                Unfinished
                    Alias("Value"); Alias("number"); Token("token", "3")
                Preceeds
                Alias("Value")
                Start("Binary")
                    Alias("Op"); Token("token", "*")
                    Unfinished
                        Alias("Value"); Alias("number"); Token("token", "4")
                    Abort
                End
            End
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
            Unfinished
                Alias("Value"); Alias("number"); Token("token", "2")
            Preceeds
            Alias("Value"); Start("Binary")
                Alias("Op"); Token("token", "+")
                Unfinished
                    Alias("Value"); Alias("number"); Token("token", "3")
                Abort
            Preceeds
            Alias("Value"); Start("Binary")
                Alias("Op"); Token("token", "+")
                Unfinished
                    Alias("Value"); Alias("number"); Token("token", "4")
                Abort
            End
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
    fn parens() {
        assert_ok(
            "(2+3)*4",
            r#"
            Unfinished
                Token("token", "(")
                Unfinished
                    Alias("Value"); Alias("number"); Token("token", "2")
                Preceeds
                Alias("Value"); Start("Binary")
                    Alias("Op"); Token("token", "+")
                    Unfinished
                        Alias("Value"); Alias("number"); Token("token", "3")
                    Abort
                End
                Token("token", ")")
            Preceeds
            Alias("Value"); Start("Binary")
                Alias("Op"); Token("token", "*")
                Unfinished
                    Alias("Value"); Alias("number"); Token("token", "4")
                Abort
            End
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
            Unfinished
                Alias("Value"); Alias("number"); Token("token", "2")
            Preceeds
            Alias("Value"); Start("Binary")
                Alias("Op"); Token("token", "+")
                Unfinished
                    Trivia(Leading, "\n ")
                    Alias("Value"); Alias("number"); Token("token", "3")
                Abort
            End
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
            Unfinished
                Alias("Value"); Alias("number"); Token("token", "2")
                Trivia(Leading, "\n ")
            Preceeds
            Alias("Value"); Start("Binary")
                Alias("Op"); Token("token", "+")
                Unfinished
                    Alias("Value"); Alias("number"); Token("token", "3")
                Abort
            End
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
            Unfinished
                Alias("Value"); Alias("number"); Token("token", "2")
                Preceeds
            Alias("Value"); Start("Binary")
                Alias("Op"); Token("token", "+")
                Unfinished
                    Alias("Value"); Alias("number"); Trivia(Trailing, " "); Token("token", "3")
                Abort
            End
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
            Unfinished
                Alias("Value"); Start("Array")
                    Trivia(Trailing, " "); Token("token", "[")
                    Unfinished
                        Alias("Value"); Alias("number"); Token("token", "2")
                        Preceeds
                    Alias("Value"); Start("Binary")
                        Alias("Op"); Token("token", "+")
                        Unfinished
                            Alias("Value"); Alias("number"); Token("token", "3")
                        Abort
                    End
                    Trivia(Trailing, " "); Token("token", ",")
                    Unfinished
                        Alias("Value"); Alias("number"); Token("token", "4")
                    Abort
                    Token("token", "]")
                End
            Abort
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
            Unfinished
                Alias("Value"); Start("Array")
                    Trivia(Trailing, " "); Token("token", "[")
                    Unfinished
                        Alias("Value"); Alias("number"); Token("token", "2")
                        Preceeds
                    Alias("Value"); Start("Binary")
                        Alias("Op"); Token("token", "+")
                        Unfinished
                            Alias("Value"); Alias("number"); Token("token", "3")
                        Abort
                    End
                    Trivia(Trailing, " "); Token("token", ",")
                    Unfinished
                        Alias("Value"); Alias("number"); Token("token", "4")
                    Abort
                    Token("token", ",")
                    Token("token", "]")
                End
            Abort
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
}
