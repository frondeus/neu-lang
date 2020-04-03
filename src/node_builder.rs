use text_size::{TextSize, TextRange};
use std::collections::BTreeSet;
use crate::node::{Node, Nodes, Name};
use crate::lexer::Lexer;
use crate::token::TokenKind;
use crate::parser::{Parser, Context};

pub struct NodeBuilder {
    names: BTreeSet<Name>,
    children: Vec<Node>,
    from: TextSize,
}

impl NodeBuilder {
    pub(crate) fn new<K, Lex>(context: &Context<Lex>) -> Self
        where
            K: TokenKind,
            Lex: Lexer<K>
    {
        Self {
            names: Default::default(),
            children: Default::default(),
            from: context.lexer().input().cursor,
        }
    }

    pub fn name(&mut self, name: Name) -> &mut Self {
        self.names.insert(name);
        self
    }

    pub fn parse<Lex, K>(&mut self, parser: impl Parser<Lex, K>, context: &mut Context<Lex>)
        where
            K: TokenKind,
            Lex: Lexer<K>,
    {
        self.add(parser.parse(context));
    }

    pub fn add(&mut self, node: Node) {
        if node.is(Nodes::Virtual) {
            let names: Vec<Name> = node
                .names
                .into_iter()
                .filter(|name| *name != Nodes::Virtual)
                .collect();
            let children = node.children.into_iter().map(|mut child| {
                if !child.is(Nodes::Trivia) {
                    child.names.extend(names.iter());
                }
                child
            });
            self.children.extend(children);
        } else {
            self.children.push(node);
        }
    }

    pub fn build<K, Lex>(self, context: &Context<Lex>) -> Node
        where
            K: TokenKind,
            Lex: Lexer<K>
    {
        let NodeBuilder {
            from,
            names,
            children,
        } = self;
        let to = context.lexer().input().cursor;
        let span = TextRange(from, to);
        Node {
            span,
            names,
            children,
        }
    }
}
