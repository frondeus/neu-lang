use std::path::Path;
use warp::Filter;

#[cfg(not(debug_assertions))]
macro_rules! resource {
    ($path: literal) => {{
        (include_bytes!($path)
            .into_iter()
            .copied()
            .collect::<Vec<_>>())
    }};
}
#[cfg(debug_assertions)]
macro_rules! resource {
    ($path: literal) => {{
        let dir: std::path::PathBuf = file!().into();
        let path = dir.parent().expect("parent dir").join($path);
        log::info!("Loading {}", path.display());
        std::fs::read(path).expect("resource")
    }};
}

pub async fn run(root: &Path, dist: &Path) {
    let neu = warp::path("neu").and(warp::fs::dir(root.join(dist)));

    let index = warp::path::end().map(|| warp::reply::html(resource!("index.html")));

    let content =
        warp::path!(String / String).map(|_, _| warp::reply::html(resource!("index.html")));

    let js = warp::path!("static" / "main.js")
        .map(|| resource!("main.js"))
        .map(|reply| warp::reply::with_header(reply, "content-type", "text/javascript"));

    let css = warp::path!("static" / "main.css")
        .map(|| resource!("main.css"))
        .map(|reply| warp::reply::with_header(reply, "content-type", "text/css"));

    let react_dev = warp::path!("static" / "react.development.js")
        .map(|| resource!("react.development.js"))
        .map(|reply| warp::reply::with_header(reply, "content-type", "text/javascript"));

    let react_dom_dev = warp::path!("static" / "react-dom.development.js")
        .map(|| resource!("react-dom.development.js"))
        .map(|reply| warp::reply::with_header(reply, "content-type", "text/javascript"));

    let icons_eot =
        warp::path!("static" / "Icons-Regular.eot").map(|| resource!("Icons-Regular.eot"));
    let icons_ttf =
        warp::path!("static" / "Icons-Regular.ttf").map(|| resource!("Icons-Regular.ttf"));
    let icons_woff =
        warp::path!("static" / "Icons-Regular.woff").map(|| resource!("Icons-Regular.woff"));
    let icons_woff2 =
        warp::path!("static" / "Icons-Regular.woff2").map(|| resource!("Icons-Regular.woff2"));

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

/*
async fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();

    env_logger::from_env(Env::default().default_filter_or("info")).init();
    log::debug!("{:?}", opts);

    let root = find_in_ancestors(opts.path, &opts.dist)?;

    run(&root, &opts.dist).await;

    Ok(())
}
 */
