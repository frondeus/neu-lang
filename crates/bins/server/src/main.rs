use warp::Filter;
use futures::{FutureExt, StreamExt};
use std::path::{PathBuf, Path};
use neu_cli::find_in_ancestors;
use clap::Clap;
use env_logger::Env;

#[derive(Debug, Clap)]
struct Opts {
    path: Option<PathBuf>,

    #[clap(short, long, default_value = ".neu")]
    dist: PathBuf
}

async fn run(root: &Path, dist: &Path) {
    let routes =
        warp::fs::dir(root.join(".neu"));

    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();

    env_logger::from_env(Env::default().default_filter_or("info")).init();
    log::debug!("{:?}", opts);

    let mut rt = tokio::runtime::Builder::new()
        .enable_all()
        .build()?;

    let root = find_in_ancestors(opts.path, &opts.dist)?;

    run(&root, &opts.dist).await;

    Ok(())
}
