pub use text_size::TextRange;

pub struct CoreNodes;
#[allow(non_upper_case_globals)]
impl CoreNodes {
    pub const Virtual: Name = Name::new("Virtual");
    pub const Root: Name = Name::new("Root");
    pub const Trivia: Name = Name::new("Trivia");
    pub const Token: Name = Name::new("Token");
    pub const Error: Name = Name::new("Error");
}

mod input;
mod lexer;
mod peekable;
mod spanned;

mod context;
mod error;
mod parser;
mod parsers;
mod state;
mod children;

#[macro_use]
mod node;
mod node_builder;

pub use input::*;
pub use lexer::*;
pub use peekable::*;
pub use spanned::*;

pub use context::*;
pub use error::*;
pub use parser::*;
pub use parsers::*;
pub use state::*;

pub use node::*;
pub use node_builder::*;
pub use children::*;
