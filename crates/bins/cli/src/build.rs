use crate::index::{Index, IndexEntry};
use anyhow::Result;
use neu_db::Diagnostician;
use neu_eval::eval;
use neu_render::db::Renderer;
use neu_syntax::ast::ArticleItem;
use neu_syntax::db::{ArticleId, FileId, Kind, Parser};
use std::io::Write;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialOrd, Ord, PartialEq, Eq)]
#[error("{0}")]
pub struct IoError(String);

impl From<anyhow::Error> for IoError {
    fn from(e: anyhow::Error) -> Self {
        Self(format!("{}", e))
    }
}

#[salsa::query_group(BuilderDatabase)]
pub trait Builder: salsa::Database + Renderer + Parser + Diagnostician {
    fn build_all(&self, root: PathBuf, dist: PathBuf) -> Result<(), IoError>;

    fn build_article(
        &self,
        kind: Kind,
        id: ArticleId,
        path: FileId,
        article_item: ArticleItem,
        articles_path: PathBuf,
    ) -> Result<IndexEntry, IoError>;
}

pub fn build(db: &mut dyn Builder, root: &Path, dist: &Path) -> Result<()> {
    scan_all(db, root)?;
    db.build_all(root.into(), dist.into())?;

    println!("Build finished");

    Ok(())
}

pub(crate) fn scan_all(db: &mut dyn Builder, root: &Path) -> Result<()> {
    let articles = glob::glob(&format!("{}/**/*.md", root.display()))?
        .map(|entry| entry.map_err(anyhow::Error::from))
        .collect::<Result<Vec<_>>>();
    let articles = articles?;

    db.set_all_neu(None.into_iter().collect());
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
    Ok(())
}

fn build_all(db: &dyn Builder, root: PathBuf, dist: PathBuf) -> Result<(), IoError> {
    build_all_inner(db, &root, &dist)?;
    Ok(())
}

fn build_all_inner(db: &dyn Builder, root: &Path, dist: &Path) -> Result<()> {
    let articles_path = root.join(dist).join("articles");
    std::fs::create_dir_all(&articles_path)?;

    let parsed_articles = db.parse_all_mds();

    let index = parsed_articles
        .into_iter()
        .map(|(kind, id, path, ast)| build_article(db, kind, id, path, ast, articles_path.clone()))
        .collect::<Result<Vec<_>, IoError>>();

    let index: Index = index?.into();
    let index_path = root.join(dist).join("index.json");
    let mut file = std::fs::File::create(index_path)?;
    file.write_all(serde_json::to_vec(&index)?.as_slice())?;

    use crate::span_ext::*;
    let diagnostics = db.all_diagnostics();
    let diagnostics = diagnostics
        .into_iter()
        .filter_map(|(path, id, error)| {
            let input = db.input_md(path.clone());
            let lines = input.lines().map(ToString::to_string).collect::<Vec<_>>();
            let parsed = db.parse_md_syntax(path.clone());
            let node = parsed.arena.get(id);
            let path = PathBuf::from(path);
            let path = path.strip_prefix(root).expect("Strip prefix");
            node.span.lines_cols(&lines).last().map(
                |LineCols {
                     line, col_start, ..
                 }| {
                    format!("{} | {}:{} | {}", path.display(), line, col_start, error)
                },
            )
        })
        .collect::<Vec<_>>();

    if !diagnostics.is_empty() {
        eprintln!("--- Diagnostics ---");
    }
    diagnostics.iter().for_each(|diagnostic| {
        eprintln!("{}", diagnostic);
    });

    let diagnostics_path = root.join(dist).join("diagnostics.json");
    let mut file = std::fs::File::create(diagnostics_path)?;
    file.write_all(serde_json::to_vec(&diagnostics)?.as_slice())?;

    Ok(())
}

fn build_article(
    db: &dyn Builder,
    kind: Kind,
    id: ArticleId,
    path: FileId,
    article_item: ArticleItem,
    articles_path: PathBuf,
) -> Result<IndexEntry, IoError> {
    let entry = build_article_inner(db, kind, id, path, article_item, &articles_path)?;
    Ok(entry)
}

fn build_article_inner(
    db: &dyn Builder,
    kind: Kind,
    id: ArticleId,
    path: FileId,
    article_item: ArticleItem,
    articles_path: &Path,
) -> Result<IndexEntry> {
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
    log::debug!("Title - {}", title);
    let item_path = kind_path.join(&format!("{}.html", id));

    // TODO: instead of rerendering it make a link to the original article.
    let rendered = db.render_md(path);

    log::debug!("To {}", item_path.display());

    let mut file = std::fs::File::create(&item_path)?;
    file.write_all(rendered.output.as_bytes())?;

    Ok(IndexEntry {
        kind,
        id,
        title: title.into(),
        path: item_path.display().to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_fs::prelude::*;
    use predicates::prelude::*;
    use anyhow::Result;
    use crate::Database;
    use std::time::Duration;

    #[test]
    fn simple() -> Result<()> {
        let md_file_a: PathBuf = "tests/a.md".into();
        let md_file_b: PathBuf = "tests/b.md".into();

        let temp = assert_fs::TempDir::new()?;
        let root = temp.path();
        let dist = PathBuf::from(".neu");

        let md_a = temp.child("a.md");
        md_a.touch()?;
        md_a.write_file(&md_file_a)?;

        let md_b = temp.child("b.md");
        md_b.touch()?;
        md_b.write_file(&md_file_b)?;


        let mut db = Database::default();
        build(&mut db, &root, &dist)?;

        temp.child(".neu").assert(predicate::path::exists());
        temp.child(".neu").child("articles").assert(predicate::path::exists());
        temp.child(".neu").child("articles").child("test").assert(predicate::path::exists());
        temp.child(".neu").child("articles").child("test").child("1234aaaa.html").assert(predicate::path::exists());
        temp.child(".neu").child("articles").child("test").child("1234bbbb.html").assert(predicate::path::exists());

        temp.close()?;
        Ok(())
    }
}