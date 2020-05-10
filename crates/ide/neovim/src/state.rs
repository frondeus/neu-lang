use crate::Buffer;

#[derive(Clone)]
pub struct State {
    pub debug_bf: Buffer,
    pub highlight_ns: i64
}

impl State {
    pub fn new(debug_bf: Buffer, highlight_ns: i64) -> Self {
        Self { debug_bf, highlight_ns }
    }
}

