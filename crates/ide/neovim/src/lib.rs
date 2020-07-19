use neu_canceled::Canceled;
use nvim_rs::compat::tokio::Compat;
use tokio::io::Stdout;
use neu_syntax::db::FileId;

pub mod diagnostic;
pub mod handler;
pub mod highlight;
pub mod span_ext;
pub mod state;

pub type Buffer<T = Compat<Stdout>> = nvim_rs::Buffer<T>;
pub type Neovim<T = Compat<Stdout>> = nvim_rs::Neovim<T>;

#[salsa::database(
    neu_syntax::db::ParserDatabase,
    neu_render::db::RendererDatabase,
    neu_eval::db::EvaluatorDatabase,
    neu_analyze::db::AnalyzerDatabase,
    neu_db::DiagnosticianDatabase
)]
#[derive(Default)]
pub struct Database {
    storage: salsa::Storage<Self>,
}
impl salsa::Database for Database {
    fn on_propagated_panic(&self) -> ! {
        Canceled::throw()
    }
}
impl salsa::ParallelDatabase for Database {
    fn snapshot(&self) -> salsa::Snapshot<Self> {
        salsa::Snapshot::new(Database {
            storage: self.storage.snapshot(),
        })
    }
}

pub type Snapshot = salsa::Snapshot<Database>;
pub type SnapshotTx = tokio::sync::mpsc::UnboundedSender<Snapshot>;

pub enum Message {
    Modified(FileId, String),
    GetSnapshot(tokio::sync::mpsc::UnboundedSender<Snapshot>),
}

pub type MessageTx = tokio::sync::mpsc::UnboundedSender<Message>;
