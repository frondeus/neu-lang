use crate::core::{Context, Lexer, Name, Node, Parser, State, NodeId, Error};
use crate::Nodes;
use std::collections::BTreeSet;
use text_size::{TextRange, TextSize};

pub struct NodeBuilder {
    names: BTreeSet<Name>,
    children: Vec<NodeId>,
    from: TextSize,
    error: Option<Error>
}

impl NodeBuilder {
    pub(crate) fn new(lexer: &Lexer) -> Self {
        Self {
            names: Default::default(),
            children: Default::default(),
            from: lexer.input().cursor,
            error: None
        }
    }

    pub fn name(&mut self, name: Name) -> &mut Self {
        self.names.insert(name);
        self
    }

    pub fn error(&mut self, error: Error) -> &mut Self {
        self.error = Some(error);
        self.name(Nodes::Error)
    }

    pub fn parse(&mut self, state: &mut State, ctx: &Context, parser: impl Parser) {
        if let Some(trivia) = ctx.trivia() {
            let node = trivia.parse(state, ctx);
            self.add(state, node);
        }

        let node = parser.parse(state, ctx);
        self.add(state, node);

        if let Some(trivia) = ctx.trivia() {
            let node = trivia.parse(state, ctx);
            self.add(state, node);
        }
    }

    pub fn add(&mut self, state: &mut State, node: Node) {
        if node.is(Nodes::Virtual) {
            let names: Vec<Name> = node
                .names
                .into_iter()
                .filter(|name| *name != Nodes::Virtual)
                .collect();
            let children = node.children.into_iter().map(|child| {
                if !state.nodes().get(child).is(Nodes::Trivia) {
                    state.nodes().get_mut(child).names.extend(names.iter());
                }
                child
            });
            self.children.extend(children);
        } else {
            let id = state.nodes().add(node);
            state.commit_errors(id);
            self.children.push(id);
        }
    }

    pub fn build(self, state: &mut State) -> Node {
        let lexer = state.lexer();
        let NodeBuilder {
            from,
            names,
            children,
            error
        } = self;
        let to = lexer.input().cursor;
        let span = TextRange(from, to);
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
