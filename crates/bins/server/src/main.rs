use warp::Filter;
use std::path::{PathBuf, Path};
use neu_cli::find_in_ancestors;
use clap::Clap;
use env_logger::Env;
use crate::resources::Resources;

#[derive(Debug, Clap)]
struct Opts {
    path: Option<PathBuf>,

    #[clap(short, long, default_value = ".neu")]
    dist: PathBuf
}

mod resources;

async fn run(root: &Path, dist: &Path) {
    let neu = warp::path("neu")
        .and( warp::fs::dir(root.join(dist)));

    let index = warp::path::end()
        .map(|| warp::reply::html(Resources::Index.load()));

    let content = warp::path!(String / String)
        .map(|_, _| warp::reply::html(Resources::Index.load()));

    let react_dev = warp::path!("static" / "react.development.js")
        .map(|| Resources::React.load())
        .map(|reply|
            warp::reply::with_header(reply, "content-type", "text/javascript"));

    let react_dom_dev = warp::path!("static" / "react-dom.development.js")
        .map(|| Resources::ReactDom.load())
        .map(|reply|
            warp::reply::with_header(reply, "content-type", "text/javascript"));

    let routes = neu
        .or(react_dev)
        .or(react_dom_dev)
        .or(content)
        .or(index);

    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();

    env_logger::from_env(Env::default().default_filter_or("info")).init();
    log::debug!("{:?}", opts);

    let root = find_in_ancestors(opts.path, &opts.dist)?;

    run(&root, &opts.dist).await;

    Ok(())
}
