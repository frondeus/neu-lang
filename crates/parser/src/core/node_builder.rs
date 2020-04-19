use crate::core::{Context, Name, Node, Parser, Lexer, State, NodeId, Error};
use crate::Nodes;
use std::collections::BTreeSet;
use text_size::TextRange;

pub struct NodeBuilder<'a, Lex: Lexer> {
    state: &'a mut State<Lex>,
    ctx: &'a Context<'a, Lex>,
    span: TextRange,
    names: BTreeSet<Name>,
    children: Vec<NodeId>,
    error: Option<Error<Lex::Token>>
}

impl<'a, Lex: Lexer> NodeBuilder<'a, Lex> {
    pub(crate) fn new(state: &'a mut State<Lex>, ctx: &'a Context<'a, Lex>) -> Self {
        let from = state.lexer().input().cursor;
        let span = TextRange(from, from);
        Self {
            state, ctx,
            span,
            names: Default::default(),
            children: Default::default(),
            error: None
        }
    }

    pub fn peek_token(&mut self) -> Option<Lex::Token> {
        use crate::core::lexer::OptionExt;
        self.state.lexer_mut().peek().as_kind()
    }

    pub fn next_token(&mut self) -> Option<crate::core::spanned::Spanned<Lex::Token>> {
        let next = self.state.lexer_mut().next();
        if let Some(next) = next.as_ref() {
            self.span = TextRange::covering(self.span, next.span);
        }
        next
    }

    pub fn name(&mut self, name: Name) -> &mut Self {
        self.names.insert(name);
        self
    }

    pub fn error(&mut self, error: Error<Lex::Token>) -> &mut Self {
        self.error = Some(error);
        self.name(Nodes::Error)
    }

    pub fn parse_mode<'b, Lex2: Lexer>(&mut self, ctx: &'b Context<'b, Lex2>, parser: impl Parser<Lex2>) {
        let mut mode_state = self.state.transform();
        let node = parser.parse(&mut mode_state, ctx);
        self.state.restore(mode_state);
        self.add(node);
    }

    pub fn parse_ctx<'b>(&mut self, ctx: &'b Context<'b, Lex>, parser: impl Parser<Lex>) {
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

    pub fn parse(&mut self, parser: impl Parser<Lex>) {
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
            names,
            children,
            error,
            state,
            mut span,
            ..
        } = self;
        for child in &children {
            let child_node = &state.nodes().get(*child);
            if child_node.is(Nodes::Trivia) { continue; }
            span = TextRange::covering(span, child_node.span);
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
