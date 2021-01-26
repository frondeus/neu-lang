use crate::index::{Index, IndexEntry};
use anyhow::Result;
use neu_db::Diagnostician;
use neu_render::db::Renderer;
use neu_syntax::ast::ArticleItem;
use neu_syntax::db::{ArticleId, FileId, FileKind, Kind, Parser};
use neu_syntax::reexport::Ast;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Arc;
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

    db.set_all_neu(Arc::default());
    db.set_all_mds(Arc::new(
        articles
            .iter()
            .map(|path| db.file_id((path.display().to_string(), FileKind::Md)))
            .collect(),
    ));

    for entry in &articles {
        let file_id = db.file_id((entry.display().to_string(), FileKind::Md));

        let file = std::fs::read_to_string(entry)?;
        let input = &file;
        db.set_input(file_id, Arc::new(input.clone()));
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
        .map(|(kind, id, path, ast)| db.build_article(kind, id, path, ast, articles_path.clone()))
        .collect::<Result<Vec<_>, IoError>>();

    let index: Index = index?.into();
    let index_path = root.join(dist).join("index.json");
    let mut file = std::fs::File::create(index_path)?;
    file.write_all(serde_json::to_vec(&index)?.as_slice())?;

    use crate::span_ext::*;
    let diagnostics = db.all_diagnostics();
    let diagnostics = diagnostics
        .into_iter()
        .filter_map(|(path, range, error)| {
            let input = db.input(path);
            let lines = input.lines().map(ToString::to_string).collect::<Vec<_>>();
            let path = db.lookup_file_id(path);
            let path = PathBuf::from(path.0);
            let path = path.strip_prefix(root).expect("Strip prefix");
            range.lines_cols(&lines).last().map(
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
    //log::info!("Building {}:{}, {:?}, {:?}. {:?}", kind, id, path, article_item, articles_path);
    let strukt = article_item
        .strukt()
        .map(|strukt| db.eval(strukt.red()))
        .and_then(|strukt_eval| strukt_eval.value.clone())
        .and_then(|value| value.into_struct());

    let title = strukt
        .as_ref()
        .and_then(|value| value.get("title"))
        .map(ToString::to_string)
        .unwrap_or_else(|| "???".to_string());

    let title = title.trim_matches('"');
    log::info!("Building {}:{} - {}", kind, id, title);
    log::debug!("Title - {}", title);

    // TODO: instead of rerendering it make a link to the original article.
    let rendered = db.render_md(path);

    let kind_path = articles_path.join(&kind);
    let item_path = kind_path.join(&format!("{}.html", id));
    log::debug!("To {}", item_path.display());
    std::fs::create_dir_all(&kind_path)?;
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
    use crate::Database;
    use anyhow::Result;
    use assert_fs::fixture::ChildPath;
    use assert_fs::prelude::*;
    use predicates::prelude::*;
    use std::time::SystemTime;

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

        let res_a = temp
            .child(".neu")
            .child("articles")
            .child("test")
            .child("1234aaaa.html");
        let res_b = temp
            .child(".neu")
            .child("articles")
            .child("test")
            .child("1234bbbb.html");

        res_a.assert(predicate::path::exists());
        res_b.assert(predicate::path::exists());

        temp.close()?;
        Ok(())
    }

    #[test]
    fn incremental_a_changed() -> Result<()> {
        let _ = env_logger::builder().is_test(true).try_init();

        let md_file_a: PathBuf = "tests/a.md".into();
        let md_file_a_modified: PathBuf = "tests/a-modified.md".into();
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

        let res_a = temp
            .child(".neu")
            .child("articles")
            .child("test")
            .child("1234aaaa.html");
        let res_b = temp
            .child(".neu")
            .child("articles")
            .child("test")
            .child("1234bbbb.html");

        let a_time = modified(&res_a)?;
        let b_time = modified(&res_b)?;

        std::thread::sleep(std::time::Duration::from_millis(100));

        md_a.write_file(&md_file_a_modified)?;
        let file = std::fs::read_to_string(md_a.path())?;
        db.set_input(
            db.file_id((md_a.path().display().to_string(), FileKind::Md)),
            Arc::new(file),
        );

        db.build_all(root.into(), dist.into())?;

        assert_eq!(modified(&res_b)?, b_time);
        assert_ne!(modified(&res_a)?, a_time);

        temp.close()?;
        Ok(())
    }

    fn modified(child: &ChildPath) -> Result<SystemTime> {
        let metadata = std::fs::metadata(child.path())?;
        let time = metadata.modified()?;
        Ok(time)
    }
}
