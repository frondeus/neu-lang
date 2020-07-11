use crate::result::RenderResult;
use std::sync::Arc;
use neu_parser::ParseResult;

#[salsa::query_group(RendererDatabase)]
pub trait Renderer {
    //fn parse_article(&self) -> ParseResult;
    fn rendered_article(&self) -> RenderResult;
}

fn parse_article(db: &impl Renderer) -> Arc<ParseResult> {
    todo!()
}

fn rendered_article(db: &impl Renderer) -> RenderResult {
    RenderResult {
        output: "Couldn't render, found errors".to_string()
    }
}
