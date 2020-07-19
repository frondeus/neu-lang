use anyhow::{Context, Error, Result};
use env_logger::Env;
use neu_nvim::handler::NeovimHandler;
use neu_nvim::{Database, Message};
use neu_syntax::db::{FileId, FileKind, Parser};
use nvim_rs::create::tokio as create;

#[tokio::main]
async fn main() -> Result<()> {
    human_panic::setup_panic!();
    env_logger::from_env(Env::default().default_filter_or("info,salsa=warn")).init();

    let (message_tx, rx) = crossbeam::channel::unbounded::<Message>();
    std::thread::spawn(move || {
        let mut db = Database::default();

        db.set_all_neu(None.into_iter().collect());
        db.set_all_mds(None.into_iter().collect());

        while let Some(msg) = rx.recv().ok() {
            match msg {
                Message::Modified(file_id, modified) => {
                    let mut all_mds = db.all_mds();
                    if !all_mds.contains(&file_id) {
                        all_mds.insert(file_id.clone());
                        db.set_all_mds(all_mds);
                    }
                    db.set_input(file_id, modified);
                }
                Message::GetSnapshot(tx) => {
                    use salsa::ParallelDatabase;
                    let snapshot = db.snapshot();
                    let _ = tx.send(snapshot);
                }
            }
        }
    });

    let handler = NeovimHandler::new(message_tx);
    let (nvim, io_handler) = create::new_parent(handler).await;

    if let Err(err) = io_handler.await.context("Error joining IO loop")? {
        if !err.is_reader_error() {
            nvim.err_writeln(&format!("Error: '{}'", err))
                .await
                .unwrap_or_else(|e| {
                    eprintln!("Well, dang... '{}'", e);
                });
        }

        if !err.is_channel_closed() {
            let e = Error::from(err);
            eprintln!("{:?}", e);
        }
    }

    Ok(())
}
