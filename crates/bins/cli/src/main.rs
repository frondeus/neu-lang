use anyhow::Result;
use neu_cli::{find_in_ancestors};
use clap::Clap;
use env_logger::Env;
use std::path::{Path, PathBuf};
use neu_syntax::ast::ArticleItem;
use std::io::Write;
use neu_eval::eval;
use serde::Serialize;

#[derive(Debug, Clap)]
struct Opts {
    path: Option<PathBuf>,

    #[clap(short, long, default_value = ".neu")]
    dist: PathBuf
}

#[derive(Debug, Serialize)]
struct IndexEntry {
    kind: String,
    id: String,
    title: String
}

fn build_article(entry: &Path, articles_path: &Path, index: &mut Vec<IndexEntry>) -> Result<()> {
    use neu_parser::State;
    use neu_syntax::{lexers::article_item_file::Lexer,
                     parsers::article_item::parser};
    use neu_render::render;

    let file = std::fs::read_to_string(entry)?;
    let input = &file;
    let lexer = Lexer::new(input);
    let mut parsed = State::parse(lexer, parser());
    let article_item = ArticleItem::from_root(parsed.root, &parsed.arena);


    if let (Some(ident), Some(id)) = (
        article_item.identifier(&parsed.arena, input),
        article_item.item_id(&parsed.arena, input)
    ) {
        log::info!("Rendering {}:{}", ident, id);
        let ident_path = articles_path.join(ident);
        std::fs::create_dir_all(&ident_path)?;

        article_item.anchor_body(&mut parsed.arena);

        let strukt = article_item.strukt.map(
            |strukt| eval(strukt, &mut parsed.arena, input)
        )
            .and_then(|strukt_eval| { strukt_eval.value })
            .and_then(|value| value.into_struct());

        let title = strukt.as_ref()
            .and_then(|value| value.get("title"))
            .map(ToString::to_string)
            .unwrap_or_else(|| "???".to_string());

        log::debug!("Title: {}", title);

        let rendered = render(article_item, &mut parsed.arena, input);

        let item_path = ident_path.join(&format!("{}.html", id));
        log::debug!("To {}", item_path.display());

        let mut file = std::fs::File::create(item_path)?;
        file.write_all(rendered.output.as_bytes())?;

        index.push(IndexEntry {
            kind: ident.into(),
            id: id.into(),
            title
        });
    }

    Ok(())
}

fn build(root: &Path, dist: &Path) -> Result<()> {

    let articles_path = root.join(dist).join("articles");
    std::fs::create_dir_all(&articles_path)?;

    let mut index = vec![];

    for entry in glob::glob(&format!("{}/**/*.md", root.display()))? {
        let entry = entry?;
        build_article(&entry, &articles_path, &mut index)?;
    }

    let index_path = root.join(dist).join("index.json");
    let mut file = std::fs::File::create(index_path)?;
    file.write_all(
        serde_json::to_vec(&index)?.as_slice()
        //format!("{:?}", index).as_bytes()
    )?;

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
