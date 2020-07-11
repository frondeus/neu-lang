use anyhow::Result;
use clap::Clap;
use env_logger::Env;
use neu_cli::find_in_ancestors;
use neu_eval::eval;
use neu_syntax::ast::ArticleItem;
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
#[allow(dead_code)] // TODO:
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

fn build_article(entry: &Path, articles_path: &Path, index: &mut Vec<IndexEntry>) -> Result<()> {
    use neu_parser::State;
    use neu_render::render;
    use neu_syntax::{lexers::article_item_file::Lexer, parsers::article_item::parser};

    let file = std::fs::read_to_string(entry)?;
    let input = &file;
    let lexer = Lexer::new(input);
    let mut parsed = State::parse(lexer, parser());
    let article_item = ArticleItem::from_root(parsed.root, &parsed.arena);

    if let (Some(ident), Some(id)) = (
        article_item.identifier(&parsed.arena, input),
        article_item.item_id(&parsed.arena, input),
    ) {
        log::info!("Rendering {}:{}", ident, id);
        let ident_path = articles_path.join(ident);
        std::fs::create_dir_all(&ident_path)?;

        article_item.anchor_body(&mut parsed.arena);

        let strukt = article_item
            .strukt
            .map(|strukt| eval(strukt, &mut parsed.arena, input))
            .and_then(|strukt_eval| strukt_eval.value)
            .and_then(|value| value.into_struct());

        let title = strukt
            .as_ref()
            .and_then(|value| value.get("title"))
            .map(ToString::to_string)
            .unwrap_or_else(|| "???".to_string());

        let title = title.trim_matches('"');
        log::debug!("Title: {}", title);

        let rendered = render(article_item, &mut parsed.arena, input);

        let item_path = ident_path.join(&format!("{}.html", id));
        log::debug!("To {}", item_path.display());

        let mut file = std::fs::File::create(&item_path)?;
        file.write_all(rendered.output.as_bytes())?;

        index.push(IndexEntry {
            kind: ident.into(),
            id: id.into(),
            title: title.into(),
            path: item_path.display().to_string(),
        });
    }

    Ok(())
}

fn build(root: &Path, dist: &Path) -> Result<()> {
    let articles_path = root.join(dist).join("articles");
    std::fs::create_dir_all(&articles_path)?;

    let mut index = vec![];

    for entry in glob::glob(&format!("{}/**/*.md", root.display()))? {
        build_article(&entry?, &articles_path, &mut index)?;
    }

    let index: Index = index.into();
    let index_path = root.join(dist).join("index.json");
    let mut file = std::fs::File::create(index_path)?;
    file.write_all(serde_json::to_vec(&index)?.as_slice())?;

    Ok(())
}

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();

    env_logger::from_env(Env::default().default_filter_or("info")).init();

    log::debug!("{:?}", opts);

    let path = opts.path;

    let root = find_in_ancestors(path, &opts.dist)?;

    build(&root, &opts.dist)?;

    Ok(())
}
