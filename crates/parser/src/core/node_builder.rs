use crate::core::{Context, Name, Node, Parser, State, NodeId, Error};
use crate::Nodes;
use std::collections::BTreeSet;
use text_size::{TextRange, TextSize};

pub struct NodeBuilder<'a> {
    state: &'a mut State,
    ctx: &'a Context<'a>,

    names: BTreeSet<Name>,
    children: Vec<NodeId>,
    from: TextSize,
    error: Option<Error>
}

impl<'a> NodeBuilder<'a> {
    pub(crate) fn new(state: &'a mut State, ctx: &'a Context<'a>) -> Self {
        let from = state.lexer().input().cursor;
        Self {
            state, ctx,

            names: Default::default(),
            children: Default::default(),
            from,
            error: None
        }
    }

    pub fn peek_token(&mut self) -> Option<crate::Token> {
        use crate::core::peekable::PeekableIterator;
        use crate::core::lexer::OptionExt;
        self.state.lexer_mut().peek().as_kind()
    }

    pub fn next_token(&mut self) -> Option<crate::core::spanned::Spanned<crate::Token>> {
        self.state.lexer_mut().next()
    }

    pub fn name(&mut self, name: Name) -> &mut Self {
        self.names.insert(name);
        self
    }

    pub fn error(&mut self, error: Error) -> &mut Self {
        self.error = Some(error);
        self.name(Nodes::Error)
    }

    pub fn parse_ctx<'b>(&mut self, ctx: &'b Context<'b>, parser: impl Parser) {
        if let Some(trivia) = ctx.trivia() {
            let node = trivia.parse(self.state, ctx);
            self.add(node);
        }

        let node = parser.parse(self.state, ctx);
        self.add(node);

        if let Some(trivia) = ctx.trivia() {
            let node = trivia.parse(self.state, ctx);
            self.add(node);
        }
    }

    pub fn parse(&mut self, parser: impl Parser) {
        self.parse_ctx(self.ctx, parser);
    }

    pub fn add(&mut self, node: Node) {
        if node.is(Nodes::Virtual) {
            let names: Vec<Name> = node
                .names
                .into_iter()
                .filter(|name| *name != Nodes::Virtual)
                .collect();
            let mut children = node.children.into_iter().map(|child| {
                if !self.state.nodes().get(child).is(Nodes::Trivia) {
                    self.state.nodes().get_mut(child).names.extend(names.iter());
                }
                child
            }).collect::<Vec<_>>();
            self.children.append(&mut children);
        } else {
            let id = self.state.nodes().add(node);
            self.state.commit_errors(id);
            self.children.push(id);
        }
    }

    pub fn build(self) -> Node {
        let NodeBuilder {
            from,
            names,
            children,
            error,
            state,
            ..
        } = self;
        let lexer = state.lexer();
        let to = lexer.input().cursor;

        let mut span = TextRange(from, to);
        for child in &children {
            let child_span = &state.nodes().get(*child).span;
            span = TextRange::covering(span, *child_span);
        }

        if let Some(error) = error {
            state.error(error);
        }
        Node {
            span,
            names,
            children,
        }
    }
}
