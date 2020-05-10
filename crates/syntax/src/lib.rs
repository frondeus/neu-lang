mod nodes;

pub mod lexers;

mod common;

#[derive(Default, Clone)]
pub struct HashCount {
    count: usize
}

impl From<()> for HashCount {
    fn from(_: ()) -> Self { Default::default() }
}
impl From<HashCount> for () {
    fn from(_: HashCount) -> Self { () }
}

pub use crate::nodes::*;

pub mod neu;
pub mod markdown;
pub mod article_item;