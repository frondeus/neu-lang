use neu_parser::NodeId;
use std::fmt;
use derive_more::Display;

#[derive(Display)]
#[display(fmt = "")]
pub enum Error {

}

pub struct RenderResult {
    pub output: String,
    pub errors: Vec<(NodeId, Error)>
}

impl RenderResult {
    pub fn display<'s, 'n>(&'n self, str: &'s str) -> DisplayRenderResult<'s, 'n> {
        DisplayRenderResult { str, result: self }
    }
}

pub struct DisplayRenderResult<'s, 'n> {
    #[allow(dead_code)]
    str: &'s str,
    result: &'n RenderResult
}

impl<'s, 'n> fmt::Display for DisplayRenderResult<'s, 'n> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "`{}`", self.result.output)?;
        if self.result.errors.is_empty() {
            write!(f, "\n\n### No Errors ###")?;
        } else {
            write!(f, "\n\n### Errors ###")?;
        }

        for (node_id, error) in self.result.errors.iter() {
            write!(f, "\n{} @ {:?}", error, node_id)?;
        }

        Ok(())
    }
}