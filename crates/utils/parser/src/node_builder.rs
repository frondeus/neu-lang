use crate::CoreNodes as Nodes;
use crate::{
    Context, Error, Name, Node, NodeId, OptionExt, Parser, PeekableIterator, State, TokenKind,
};
use std::collections::BTreeSet;
use text_size::TextRange;

pub struct NodeBuilder<'a, Tok: TokenKind> {
    state: &'a mut State<Tok>,
    ctx: &'a Context<'a, Tok>,
    span: TextRange,
    names: BTreeSet<Name>,
    children: Vec<NodeId>,
    error: Option<Error<Tok>>,
}

impl<'a, Tok: TokenKind> NodeBuilder<'a, Tok> {
    pub fn new(state: &'a mut State<Tok>, ctx: &'a Context<'a, Tok>) -> Self {
        let from = state.lexer().input().cursor();
        let span = TextRange::new(from, from);
        Self {
            state,
            ctx,
            span,
            names: Default::default(),
            children: Default::default(),
            error: None,
        }
    }

    pub fn peek_token(&mut self) -> Option<Tok> {
        self.state.lexer_mut().peek().as_kind()
    }

    pub fn state(&self) -> &State<Tok> {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut State<Tok> {
        &mut self.state
    }

    pub fn span(&self) -> TextRange {
        self.span
    }

    pub fn set_span(&mut self, span: TextRange) {
        self.span = span;
    }

    pub fn next_token(&mut self) -> Option<crate::spanned::Spanned<Tok>> {
        let next = self.state.lexer_mut().next();
        if let Some(next) = next.as_ref() {
            self.span = self.span.cover(next.span);
        }
        next
    }

    pub fn name(&mut self, name: Name) -> &mut Self {
        self.names.insert(name);
        self
    }

    pub fn error(&mut self, error: Error<Tok>) -> &mut Self {
        self.error = Some(error);
        self.name(Nodes::Error)
    }

    pub fn parse_mode<'b, Tok2>(&mut self, ctx: &'b Context<'b, Tok2>, parser: impl Parser<Tok2>)
    where
        Tok2: TokenKind,
        Tok2::Extra: Into<Tok::Extra>,
        Tok::Extra: Into<Tok2::Extra>,
    {
        let mut mode_state = self.state.transform();
        let node = parser.parse(&mut mode_state, ctx);
        self.state.restore(mode_state);
        self.add(node);
    }

    pub fn parse_ctx<'b>(&mut self, ctx: &'b Context<'b, Tok>, parser: impl Parser<Tok>) {
        if let Some(trivia) = ctx.leading_trivia() {
            let node = trivia.parse(self.state, ctx);
            self.add(node);
        }

        let node = parser.parse(self.state, ctx);
        self.add(node);

        if let Some(trivia) = ctx.trailing_trivia() {
            let node = trivia.parse(self.state, ctx);
            self.add(node);
        }
    }

    pub fn parse(&mut self, parser: impl Parser<Tok>) {
        self.parse_ctx(self.ctx, parser);
    }

    pub fn add(&mut self, node: Node) {
        if node.is(Nodes::Virtual) {
            let names: Vec<Name> = node
                .names
                .into_iter()
                .filter(|name| *name != Nodes::Virtual)
                .collect();
            let mut children = node
                .children
                .into_iter()
                .map(|child| {
                    if !self.state.nodes().get(child).is(Nodes::Trivia) {
                        let child_node = self.state.nodes().get_mut(child);
                        child_node.names.extend(names.iter());
                    }
                    child
                })
                .collect::<Vec<_>>();
            self.children.append(&mut children);
        } else if node.is(Nodes::Error) || !node.children.is_empty() || !node.span.is_empty() {
            let id = self.state.nodes().add(node);
            self.state.commit_errors(id);
            self.children.push(id);
        }
    }

    pub fn build(self) -> Node {
        let NodeBuilder {
            names,
            children,
            error,
            state,
            mut span,
            ..
        } = self;
        for child in &children {
            let child_node = &state.nodes().get(*child);
            if child_node.is(Nodes::Trivia) {
                continue;
            }
            span = span.cover(child_node.span);
        }

        if let Some(error) = error {
            state.error(error);
        }
        Node {
            span,
            names,
            children,
            parent: Default::default(),
        }
    }
}
