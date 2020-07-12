use crate::resources::Resources;
use clap::Clap;
use env_logger::Env;
use neu_cli::find_in_ancestors;
use std::path::{Path, PathBuf};
use warp::Filter;

#[derive(Debug, Clap)]
struct Opts {
    path: Option<PathBuf>,

    #[clap(short, long, default_value = ".neu")]
    dist: PathBuf,
}

mod resources;

async fn run(root: &Path, dist: &Path) {
    let neu = warp::path("neu").and(warp::fs::dir(root.join(dist)));

    let index = warp::path::end().map(|| warp::reply::html(Resources::Index.load()));

    let content =
        warp::path!(String / String).map(|_, _| warp::reply::html(Resources::Index.load()));

    let js = warp::path!("static" / "main.js")
        .map(|| Resources::Js.load())
        .map(|reply| warp::reply::with_header(reply, "content-type", "text/javascript"));

    let css = warp::path!("static" / "main.css")
        .map(|| Resources::Css.load())
        .map(|reply| warp::reply::with_header(reply, "content-type", "text/css"));

    let react_dev = warp::path!("static" / "react.development.js")
        .map(|| Resources::React.load())
        .map(|reply| warp::reply::with_header(reply, "content-type", "text/javascript"));

    let react_dom_dev = warp::path!("static" / "react-dom.development.js")
        .map(|| Resources::ReactDom.load())
        .map(|reply| warp::reply::with_header(reply, "content-type", "text/javascript"));

    let icons_eot = warp::path!("static" / "Icons-Regular.eot").map(|| Resources::IconsEot.load());
    let icons_ttf = warp::path!("static" / "Icons-Regular.ttf").map(|| Resources::IconsTTF.load());
    let icons_woff = warp::path!("static" / "Icons-Regular.woff").map(|| Resources::IconsWOFF.load());
    let icons_woff2 = warp::path!("static" / "Icons-Regular.woff2").map(|| Resources::IconsWOFF2.load());
        //.map(|reply| warp::reply::with_header(reply, "content-type", "text/javascript"));

    let routes = neu
        .or(js)
        .or(css)
        .or(react_dev)
        .or(react_dom_dev)
        .or(icons_eot)
        .or(icons_ttf)
        .or(icons_woff)
        .or(icons_woff2)
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
