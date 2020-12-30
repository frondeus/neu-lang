use std::marker::PhantomData;

use crate::{NodeBuilder, Parser, TokenKind};

pub enum Assoc {
    Right,
    Left
}

pub struct Pratt<N, BP, F, Tok> {
    next: N,
    bp: BP,
    f: F,
    rbp: i32,
    _phantom: PhantomData<Tok>
}

impl<'s, N, BP, F, Tok> Clone for Pratt<N, BP, F, Tok>
where Tok: TokenKind<'s>,
      N: Clone + Parser<'s, Tok>,
      BP: Clone + Fn(Option<Tok>) -> Option<(Assoc, i32)>,
      F: Clone + for<'c> Fn(NodeBuilder<'s, 'c, Tok>, Option<Tok>) -> NodeBuilder<'s, 'c, Tok>
{
    fn clone(&self) -> Self {
        Self {
            next: self.next.clone(),
            bp: self.bp.clone(),
            f: self.f.clone(),
            rbp: self.rbp,
            _phantom: PhantomData
        }
    }
}

impl <'s, N, BP, F, Tok> Pratt<N, BP, F, Tok>
where
    Tok: TokenKind<'s>,
    N: Clone + Parser<'s, Tok>,
    BP: Clone + Fn(Option<Tok>) -> Option<(Assoc, i32)>,
    F: Clone + for<'c> Fn(NodeBuilder<'s, 'c, Tok>, Option<Tok>) -> NodeBuilder<'s, 'c, Tok>
{
    pub fn new(next: N, bp: BP, f: F) -> Self {
        Self {
            next,
            bp,
            f,
            rbp: 0,
            _phantom: PhantomData
        }
    }

    pub fn rbp(&self, rbp: i32) -> Self {
        let mut new = self.clone();
        new.rbp = rbp - 1;
        new
    }

    pub fn parser(&self) -> impl Parser<'s, Tok> + Clone {
        use crate::parsers::node;

        let opt = self.clone();
        node(move |builder| {
            let ctx = builder.get_ctx();
            let (mut left, state) = builder.parse(opt.next.clone());

            let mut builder = state.builder(ctx).node();

            loop {
                let op_token = builder.peek_token();
                let (op_assoc, op_bp) = match (opt.bp)(op_token.as_ref().copied()) {
                    Some(op) if op.1 > opt.rbp => op,
                    _ => {
                        break (left, builder.abort());
                    }
                };

                let new_op_bp = match op_assoc {
                    Assoc::Left => op_bp + 1,
                    Assoc::Right => op_bp - 1,
                };

                builder = builder
                    .add(left);

                let (new_left, state) =
                    (opt.f)(builder, op_token)
                    .parse(opt.rbp(new_op_bp).parser())
                    .finish();

                left = new_left;

                builder = state.builder(ctx).node();
            }
        })
    }
}
