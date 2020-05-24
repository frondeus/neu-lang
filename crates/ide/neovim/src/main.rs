use anyhow::{Context, Error, Result};
use neu_nvim::handler::NeovimHandler;
use nvim_rs::create::tokio as create;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let handler = NeovimHandler::default();

    let (nvim, io_handler) = create::new_parent(handler).await;

    match io_handler.await.context("Error joining IO loop")? {
        Err(err) => {
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
        Ok(()) => {}
    }
    Ok(())
}
