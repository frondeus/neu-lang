mod hunk;
mod line;
mod processor;
mod context;

#[cfg(feature = "display")]
mod display;

#[cfg(feature = "patch")]
mod patch;

pub use crate::hunk::Hunk;
pub use crate::line::{Line, LineKind};
pub use crate::processor::Processor;
pub use crate::context::Context;

#[cfg(feature = "display")]
pub use crate::display::DisplayOptions;

#[cfg(feature = "patch")]
pub use crate::patch::PatchOptions;

use std::io;

pub struct Comparison<'a> {
    pub(crate) left: &'a [&'a str],
    pub(crate) right: &'a [&'a str],
    pub(crate) context_radius: usize
}

impl<'a> Comparison<'a> {
    pub fn new(left: &'a [&'a str], right: &'a [&'a str]) -> Self {
        Self { left, right, context_radius: 3 }
    }

    pub fn hunks(&self) -> io::Result<Vec<Hunk<'a>>> {
        let mut processor = Processor::new(&self.left, &self.right, self.context_radius);
        {
            let mut replace = diffs::Replace::new(&mut processor);
            diffs::patience::diff(&mut replace, self.left, 0, self.left.len(), self.right, 0, self.right.len())?;
        }
        Ok(processor.result())
    }
}


/*
pub fn diff(text1: &[String], text2: &[String], context_radius: usize) -> io::Result<Vec<String>> {
    let result = diff_hunks(text1, text2, context_radius)?
        .into_iter()
        .map(|hunk| format!("{}", hunk))
        .collect();
    Ok(result)
}
*/
