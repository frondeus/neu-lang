use std::path::Path;
use warp::Filter;
use futures::{StreamExt, FutureExt};
use tokio::sync::mpsc::{UnboundedSender, UnboundedReceiver};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::collections::HashMap;
use warp::ws::Message;
use std::sync::Arc;
use tokio::sync::RwLock;

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

static NEXT_USER_ID: AtomicUsize = AtomicUsize::new(1);

type WsSockets =  Arc<RwLock< HashMap<usize, UnboundedSender< Result<Message, warp::Error> >> >>;

pub async fn run(root: &Path, dist: &Path, hot_rx: UnboundedReceiver<()>) {
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

    let ws_sockets: WsSockets = Default::default();

    let wss = ws_sockets.clone();
    tokio::task::spawn(async move {
        let mut hot_rx = hot_rx;
        while hot_rx.next().await.is_some() {
            for (_, tx) in wss.read().await
                .iter() {
                let s: String = "Reloaded".into();
                if let Err(_e) = tx.send( Ok(Message::text(s)) ) {
                    log::error!("Could not send ws hotreload");
                }
            }
        }
    });

    let ws_sockets = warp::any().map(move || ws_sockets.clone());

    let hotreload = warp::path("hotreload")
        .and(warp::ws())
        .and(ws_sockets)
        .map(|ws: warp::ws::Ws, ws_sockets: WsSockets| {
            ws.on_upgrade(move |socket| {
                async move {
                    let (user_ws_tx, mut user_ws_rx) = socket.split();
                    let my_id = NEXT_USER_ID.fetch_add(1, Ordering::Relaxed);
                    log::info!("New websocket connection: {}", my_id);

                    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
                    tokio::task::spawn(rx
                        .forward(user_ws_tx).map(|result| {
                        if let Err(e) = result {
                            eprintln!("websocket send error: {}", e);
                        }
                    }));
                    ws_sockets.write().await.insert(my_id, tx);
                    while user_ws_rx.next().await.is_some() { }
                    log::info!("websocket disconnected: {}", my_id);
                    ws_sockets.write().await.remove(&my_id);

                }
            })
        });

    let routes = neu
        .or(js)
        .or(css)
        .or(react_dev)
        .or(react_dom_dev)
        .or(icons_eot)
        .or(icons_ttf)
        .or(icons_woff)
        .or(icons_woff2)
        .or(hotreload)
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
