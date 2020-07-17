use anyhow::Result;
use clap::Clap;
use env_logger::Env;
use std::path::PathBuf;

pub(crate) use neu_cli::*;

#[derive(Debug, Clap)]
struct Opts {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Clap, Clone)]
enum Command {
    Build {
        path: Option<PathBuf>,

        #[clap(short, long, default_value = ".neu")]
        dist: PathBuf,
    },
    Watch {
        path: Option<PathBuf>,

        #[clap(short, long, default_value = ".neu")]
        dist: PathBuf,
    },
    Serve {
        path: Option<PathBuf>,

        #[clap(short, long, default_value = ".neu")]
        dist: PathBuf,
    },
}

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();

    env_logger::from_env(Env::default().default_filter_or("info,salsa=warn")).init();

    log::debug!("{:?}", opts);

    let mut db = Database::default();

    match opts.command {
        Command::Build { path, dist } => {
            let root = find_in_ancestors(path, &dist)?;
            build::build(&mut db, &root, &dist)?;
        }
        Command::Watch { path, dist } => {
            let root = find_in_ancestors(path, &dist)?;
            watch::watch(&mut db, &root, &dist, None)?;
        }
        Command::Serve { path, dist } => {
            let root = find_in_ancestors(path, &dist)?;
            let r = root.clone();
            let d = dist.clone();
            let rt = tokio::runtime::Builder::new()
                .threaded_scheduler()
                .enable_all()
                .build()
                .unwrap();
            let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<()>();
            rt.spawn(async move {
                server::run(&r, &d, rx).await;
            });
            watch::watch(&mut db, &root, &dist, Some(tx))?;
        }
    }

    Ok(())
}
