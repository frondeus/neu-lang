#![allow(dead_code)]
use crate::result::RenderResult;
use neu_parser::ParseResult;

#[salsa::query_group(RendererDatabase)]
trait Renderer {
    //fn parse_article(&self) -> ParseResult;
    fn rendered_article(&self) -> RenderResult;
}

fn parse_article(_db: &impl Renderer) -> ParseResult {
    todo!()
}

fn rendered_article(_db: &impl Renderer) -> RenderResult {
    todo!()
}
