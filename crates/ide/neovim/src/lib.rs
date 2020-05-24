use nvim_rs::compat::tokio::Compat;
use tokio::io::Stdout;

pub mod diagnostic;
pub mod handler;
pub mod highlight;
pub mod span_ext;
pub mod state;

pub type Buffer<T = Compat<Stdout>> = nvim_rs::Buffer<T>;
pub type Neovim<T = Compat<Stdout>> = nvim_rs::Neovim<T>;
