use anyhow::Result;
use clap::Clap;
use env_logger::Env;
use neu_cli::find_in_ancestors;
use neu_eval::eval;
use neu_render::db::Renderer;
use neu_syntax::ast::ArticleItem;
use neu_syntax::db::Parser;
use serde::Serialize;
use std::collections::BTreeMap;
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Debug, Clap)]
struct Opts {
    path: Option<PathBuf>,

    #[clap(short, long, default_value = ".neu")]
    dist: PathBuf,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)] // TODO: Finish me
enum Tree {
    Dir(String, Vec<Tree>),
    File(usize),
    None,
}

#[derive(Debug, Serialize)]
struct Index {
    data: Vec<IndexEntry>,
    abc: BTreeMap<char, Vec<usize>>,
    kind: BTreeMap<String, Vec<usize>>,
    project: Tree,
}

impl From<Vec<IndexEntry>> for Index {
    fn from(vec: Vec<IndexEntry>) -> Self {
        let mut kind: BTreeMap<String, Vec<usize>> = BTreeMap::default();
        let mut abc: BTreeMap<char, Vec<usize>> = BTreeMap::default();
        let project: Tree = Tree::None;

        vec.iter().enumerate().for_each(|(idx, entry)| {
            kind.entry(entry.kind.clone()).or_default().push(idx);

            abc.entry(entry.title.chars().next().unwrap_or(' '))
                .or_default()
                .push(idx);
        });

        let data = vec;
        Self {
            data,
            abc,
            kind,
            project,
        }
    }
}

#[derive(Debug, Serialize, Clone)]
struct IndexEntry {
    kind: String,
    id: String,
    title: String,
    path: String,
}

fn build_article(
    db: &mut dyn Renderer,
    (kind, id, path, article_item): (String, String, String, ArticleItem),
    articles_path: &Path,
    index: &mut Vec<IndexEntry>,
) -> Result<()> {
    log::debug!("Rendering {}:{}", kind, id);
    let kind_path = articles_path.join(&kind);
    std::fs::create_dir_all(&kind_path)?;

    let input = db.input_md(path.clone());
    let mut parsed = db.parse_md_syntax(path.clone());

    let strukt = article_item
        .strukt
        .map(|strukt| eval(strukt, &mut parsed.arena, &input))
        .and_then(|strukt_eval| strukt_eval.value)
        .and_then(|value| value.into_struct());

    let title = strukt
        .as_ref()
        .and_then(|value| value.get("title"))
        .map(ToString::to_string)
        .unwrap_or_else(|| "???".to_string());

    let title = title.trim_matches('"');
    log::info!("Rendering {}:{} - {}", kind, id, title);
    let item_path = kind_path.join(&format!("{}.html", id));

    /*
    This is both bug and a feature.
    If i can make my js to scroll to requested subarticle then ill leave it as it is.

    Why this is a bug:
    Each subarticle renders whole article.
    So if in one article i have 100 subarticles, it renders 100 x 100 subarticles.

    On the other hand is quite handy. My WCU takes around 1.4 MB of 61 subarticles.
    */
    let rendered = db.render_md(path);

    log::debug!("To {}", item_path.display());

    let mut file = std::fs::File::create(&item_path)?;
    file.write_all(rendered.output.as_bytes())?;

    index.push(IndexEntry {
        kind,
        id,
        title: title.into(),
        path: item_path.display().to_string(),
    });

    Ok(())
}

fn build(root: &Path, dist: &Path) -> Result<()> {
    let articles_path = root.join(dist).join("articles");
    std::fs::create_dir_all(&articles_path)?;

    let mut db = Database::default();

    let mut index = vec![];

    let articles = glob::glob(&format!("{}/**/*.md", root.display()))?
        .map(|entry| entry.map_err(anyhow::Error::from))
        .collect::<Result<Vec<_>>>();
    let articles = articles?;

    db.set_all_mds(
        articles
            .iter()
            .map(|path| path.display().to_string())
            .collect(),
    );

    for entry in &articles {
        let entry_str = entry.display().to_string();

        let file = std::fs::read_to_string(entry)?;
        let input = &file;
        db.set_input_md(entry_str.clone(), input.clone());
    }

    let parsed_articles = db.parse_all_mds();

    for parsed in parsed_articles {
        build_article(&mut db, parsed, &articles_path, &mut index)?;
    }

    let index: Index = index.into();
    let index_path = root.join(dist).join("index.json");
    let mut file = std::fs::File::create(index_path)?;
    file.write_all(serde_json::to_vec(&index)?.as_slice())?;

    Ok(())
}

#[salsa::database(neu_syntax::db::ParserDatabase, neu_render::db::RendererDatabase)]
#[derive(Default)]
struct Database {
    storage: salsa::Storage<Self>,
}
impl salsa::Database for Database {}

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();

    env_logger::from_env(Env::default().default_filter_or("info,salsa=warn")).init();

    log::debug!("{:?}", opts);

    let path = opts.path;

    let root = find_in_ancestors(path, &opts.dist)?;

    build(&root, &opts.dist)?;

    Ok(())
}
