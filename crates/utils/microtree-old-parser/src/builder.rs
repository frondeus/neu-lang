use crate::{Context, Error, Lexer, Parser, SmolStr, State, TokenKind, Trivia};
use microtree::{Green, Name};
use text_size::TextRange;
use std::collections::BTreeSet;

impl<'source, Fun, Tok> Parser<'source, Tok> for Fun
where
    Tok: TokenKind<'source>,
    Fun: FnMut(Builder<'source, '_, Tok>) -> (Option<Green>, State<'source, Tok>),
{
    fn parse(&mut self, state: State<'source, Tok>, ctx: &Context<'source, '_, Tok>) -> (Option<Green>, State<'source, Tok>) {
        self(state.builder(ctx))
    }
}

pub struct Builder<'source, 'ctx, Tok: TokenKind<'source>> {
    pub(crate) state: State<'source, Tok>, pub(crate) ctx: &'ctx
    Context<'source, 'ctx, Tok>,
    pub(crate) names: BTreeSet<Name>,
}

impl<'source, 'ctx, Tok: TokenKind<'source>> Builder<'source, 'ctx, Tok> {
    pub(crate) fn new(state: State<'source, Tok>, ctx: &'ctx Context<'source, 'ctx, Tok>) -> Self {
        Self {
            state,
            ctx,
            names: Default::default(),
        }
    }
    pub fn name(mut self, name: Name) -> Self {
        self.names.insert(name);
        self
    }
    pub fn peek_token(&mut self) -> Option<Tok> {
        let saved = self.state.lexer_mut().clone();
        let ctx = self.ctx;

        let _leading = Self::handle_trivia(ctx.leading_trivia, self.state.lexer_mut());

        let peeked = self.state.lexer_mut().peek_token();
        *self.state.lexer_mut() = saved;
        peeked
    }
    pub fn node(self) -> NodeBuilder<'source, 'ctx, Tok> {
        NodeBuilder::new(self)
    }
    pub fn set_ctx(mut self, ctx: &'ctx Context<'source, 'ctx, Tok>) -> Self {
        self.ctx = ctx;
        self
    }

    pub fn get_ctx(&self) -> &'ctx Context<'source, 'ctx, Tok> {
        self.ctx
    }

    pub fn none(self) -> (Option<Green>, State<'source, Tok>) {
        (None, self.state)
    }

    pub fn parse_mode<Tok2>(self,
                            mut parser: impl Parser<'source, Tok2>,
                            ctx: impl Into<Option<&'ctx Context<'source, 'ctx, Tok2>>>) -> (Option<Green>, State<'source, Tok>)
    where
        Tok2: TokenKind<'source> + 'ctx,
        Tok::Extras: Into<Tok2::Extras>,
        Tok2::Extras: Into<Tok::Extras>,
    {
        let default_ctx = Context::default();
        let inner_ctx = ctx.into().unwrap_or(&default_ctx);
        let Self {
            state,
            ..
        } = self;

        let state: State<Tok2> = state.morph();

        let (res, state) = parser.parse(state, &inner_ctx);

        let state: State<Tok> = state.morph();

        (res, state)
    }

    pub fn parse(self, mut parser: impl Parser<'source, Tok>) -> (Option<Green>, State<'source, Tok>) {
        let Self { state, names, ctx } = self;
        let (green, mut state) = parser.parse(state, ctx);

        let mut aliases = names.into_iter();

        let mut node = green;

        let node = loop {
            let alias = aliases.next();
            node = match (node, alias) {
                (None, _) => break None,
                (node, None) => break node,
                (Some(n), Some(alias)) => Some(state.cache().alias(alias, n)),
            }
        };

        (node, state)
    }

    pub fn error(self, desc: impl ToString) -> (Option<Green>, State<'source, Tok>) {
        let Self {
            mut state, names, ..
        } = self;

        let token = state.lexer_mut().next();
        let (range, value) = match token {
            Some(token) => {
                (token.range,
                token.value)
            },
            None => {
                let range = state.lexer_mut().span();
                (range, Default::default())
            }
        };

        let error = Error::new(desc, range);

        state.add_error(error);

        let mut node = state.cache().token(Name::new("error"), value);
        for alias in names {
            node = state.cache().alias(alias, node);
        }

        (Some(node), state)
    }

    pub fn state_mut(&mut self) -> &mut State<'source, Tok> {
        &mut self.state
    }

    fn handle_trivia(
        trivia: Option<&'ctx dyn Trivia<'source, Tok>>,
        lexer: &mut Lexer<'source, Tok>
    ) -> SmolStr {
        match trivia {
            None => Default::default(),
            Some(trivia) => {
                let start = lexer.span().end();
                trivia.parse(lexer);
                let end = lexer.span().end();
                let text_range = TextRange::new(start, end);
                lexer.text_for_span(text_range)
            }
        }
    }

    pub fn token(self) -> (Option<Green>, State<'source, Tok>) {
        let Self { mut state, names, ctx } = self;
        let mut names = names.into_iter();

        let leading = Self::handle_trivia(ctx.leading_trivia, state.lexer_mut());

        let value = state.lexer_mut().next().map(|t| t.value);

        let trailing = Self::handle_trivia(ctx.trailing_trivia, state.lexer_mut());

        let node = match value {
            None => {
                None
            }
            Some(value) => {
                let name = names.next().unwrap_or_default();

                Some(state.cache().with_trivia(name, leading, value, trailing))
            }
        };

        if let Some(mut node) = node {

            let aliases = names;
            for alias in aliases {
                node = state.cache().alias(alias, node);
            }

            (Some(node), state)
        } else {
            (None, state)
        }
    }
}

pub struct NodeBuilder<'source, 'ctx, Tok: TokenKind<'source>> {
    state: State<'source, Tok>,
    ctx: &'ctx Context<'source, 'ctx, Tok>,
    names: BTreeSet<Name>,
    children: Vec<Green>,
}

impl<'source, 'ctx, Tok: TokenKind<'source>> NodeBuilder<'source, 'ctx, Tok> {
    pub(crate) fn new(Builder { state, names, ctx }: Builder<'source, 'ctx, Tok>) -> Self {
        Self {
            state,
            names,
            ctx,
            children: Default::default(),
        }
    }

    pub fn name(mut self, name: Name) -> Self {
        self.names.insert(name);
        self
    }

    pub fn peek_token(&mut self) -> Option<Tok> {
        let saved = self.state.lexer_mut().clone();
        let ctx = self.ctx;

        let _leading = Builder::handle_trivia(ctx.leading_trivia, self.state.lexer_mut());

        let peeked = self.state.lexer_mut().peek_token();
        *self.state.lexer_mut() = saved;
        peeked
    }

    pub fn state_mut(&mut self) -> &mut State<'source, Tok> {
        &mut self.state
    }

    pub fn set_ctx(mut self, ctx: &'ctx Context<'source, 'ctx, Tok>) -> Self {
        self.ctx = ctx;
        self
    }

    pub fn add(mut self, node: Option<Green>) -> Self {
        if let Some(node) = node {
            self.children.push(node);
        }
        self
    }


    pub fn parse(mut self, mut parser: impl Parser<'source, Tok>) -> Self {
        let (res, state) = parser.parse(self.state, self.ctx);
        self.state = state;
        self.add(res)
    }

    pub fn parse_mode<Tok2>(self,
                            mut parser: impl Parser<'source, Tok2>,
                            ctx: impl Into<Option<&'ctx Context<'source, 'ctx, Tok2>>>) -> Self
    where
        Tok2: TokenKind<'source> + 'ctx,
        Tok::Extras: Into<Tok2::Extras>,
        Tok2::Extras: Into<Tok::Extras>,
    {
        let default_ctx = Context::default();
        let inner_ctx = ctx.into().unwrap_or(&default_ctx);
        let Self {
            state,
            ctx,
            names,
            mut children,
        } = self;

        let state: State<Tok2> = state.morph();


        let (res, state) = parser.parse(state, &inner_ctx);

        let state: State<Tok> = state.morph();

        if let Some(res) = res {
            children.push(res);
        }

        Self {
            state,
            ctx,
            names,
            children,
        }
    }

    pub(crate) fn abort(self) -> State<'source, Tok> {
        let Self {
            state, ..
        } = self;

        state
    }

    pub fn finish(self) -> (Option<Green>, State<'source, Tok>) {
        let Self {
            mut state,
            names,
            children,
            ..
        } = self;

        let mut names = names.into_iter();
        let name = names.next().unwrap_or_default();
        let aliases = names;

        let mut node = state.cache().node(name, children);
        for alias in aliases {
            node = state.cache().alias(alias, node);
        }

        (Some(node), state)
    }
}
