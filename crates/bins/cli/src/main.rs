use anyhow::Result;
use neu_cli::{find_in_ancestors};
use clap::Clap;
use env_logger::Env;
use std::path::{Path, PathBuf};
use neu_syntax::ast::ArticleItem;
use std::io::Write;

#[derive(Debug, Clap)]
struct Opts {
    path: Option<PathBuf>,

    #[clap(short, long, default_value = ".neu")]
    dist: PathBuf
}

fn build(root: &Path) -> Result<()> {
    use neu_parser::State;
    use neu_syntax::{lexers::article_item_file::Lexer,
                     parsers::article_item::parser};
    use neu_render::render;

    let articles_path = root.join(".neu").join("articles");
    std::fs::create_dir_all(&articles_path)?;

    for entry in glob::glob(&format!("{}/**/*.md", root.display()))? {
        let entry = entry?;
        let file = std::fs::read_to_string(entry)?;
        let input = &file;
        let lexer = Lexer::new(input);
        let mut parsed = State::parse(lexer, parser());
        let article_item = ArticleItem::build(parsed.root, &parsed.arena);


        if let (Some(ident), Some(id)) = (
            article_item.identifier(&parsed.arena, input),
            article_item.item_id(&parsed.arena, input)
        ) {
            let ident_path = articles_path.join(ident);
            std::fs::create_dir_all(&ident_path)?;

            let rendered = render(article_item, &mut parsed.arena, input);

            let item_path = ident_path.join(&format!("{}.html", id));

            let mut file = std::fs::File::create(item_path)?;
            file.write_all(rendered.output.as_bytes())?;
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();

    env_logger::from_env(Env::default().default_filter_or("info")).init();

    log::debug!("{:?}", opts);

    let path = opts.path;

    let root = find_in_ancestors(path, opts.dist)?;

    build(&root)?;

    Ok(())
}
