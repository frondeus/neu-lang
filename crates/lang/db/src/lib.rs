use std::collections::BTreeMap;

use neu_canceled::Canceled;
use neu_diagnostics::Diagnostic;
use neu_render::db::Renderer;
use neu_syntax::{
    ast::{ArticleItem, MainArticle},
    db::{FileId, Parser},
    reexport::Ast,
    reexport::{Red, TextRange, TextSize},
};

#[salsa::query_group(DiagnosticianDatabase)]
pub trait Diagnostician: salsa::Database + Parser + Renderer {
    fn all_diagnostics(&self) -> Vec<(FileId, TextRange, Diagnostic)>;
    fn all_diagnostics_sorted(&self) -> Vec<(FileId, TextRange, Diagnostic)>;
}

fn all_diagnostics(db: &dyn Diagnostician) -> Vec<(FileId, TextRange, Diagnostic)> {
    Canceled::cancel_if(db.salsa_runtime());

    let mut diagnostics: Vec<_> = db
        .all_neu()
        .iter()
        .flat_map(|path| {
            let parsed = db.parse_syntax(*path);
            let red = Red::root(parsed.root.clone());
            let evaled = db.eval(red);

            parsed
                .errors
                .iter()
                .map(|(range, error)| (*range, error))
                .chain(evaled.errors.iter())
                .map(|(range, diagnostic)| (*path, range, diagnostic.clone()))
                .collect::<Vec<_>>()
        })
        .collect();

    let md: Vec<_> = db
        .all_mds()
        .iter()
        .flat_map(|path| {
            let parsed = db.parse_syntax(*path);
            let red = Red::root(parsed.root.clone());
            let item = MainArticle::new(red.clone());
            let evaled = db.eval(red);
            let rendered = match item {
                Some(item) => {
                    let ast = ArticleItem::from(item);
                    db.render_ast(ast)
                }
                None => Default::default(),
            };

            parsed
                .errors
                .iter()
                .map(|(range, error)| (*range, error))
                .chain(evaled.errors.iter())
                .chain(rendered.errors.iter())
                .map(|(range, diagnostic)| (*path, range, diagnostic.clone()))
                .collect::<Vec<_>>()
        })
        .collect();

    diagnostics.extend(md);
    diagnostics
}

fn all_diagnostics_sorted(db: &dyn Diagnostician) -> Vec<(FileId, TextRange, Diagnostic)> {
    Canceled::cancel_if(db.salsa_runtime());

    let diagnostics = db.all_diagnostics();
    if diagnostics.is_empty() {
        return Default::default();
    }
    let mut sorted: BTreeMap<FileId, BTreeMap<TextSize, (TextRange, Diagnostic)>> =
        Default::default();

    diagnostics
        .into_iter()
        .for_each(|(path, range, diagnostic)| {
            sorted
                .entry(path)
                .or_default()
                .insert(range.start(), (range, diagnostic));
        });

    sorted
        .into_iter()
        .flat_map(|(path, diagnostics)| {
            diagnostics
                .into_iter()
                .map(move |(_, (range, diagnostic))| (path, range, diagnostic))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    use neu_syntax::db::FileKind;
    use std::sync::Arc;

    #[salsa::database(
        neu_render::db::RendererDatabase,
        neu_eval::db::EvaluatorDatabase,
        neu_analyze::db::AnalyzerDatabase,
        neu_syntax::db::ParserDatabase,
        DiagnosticianDatabase
    )]
    #[derive(Default)]
    struct TestDb {
        storage: salsa::Storage<Self>,
    }

    impl salsa::Database for TestDb {}

    #[test]
    fn neu_errors_tests() {
        test_runner::test_snapshots("neu", "errors", |input| {
            let mut db = TestDb::default();
            //let path: FileId = ("test.neu".into(), FileKind::Neu);
            let path = db.file_id(("test.neu".into(), FileKind::Neu));
            db.set_all_mds(Default::default());
            db.set_all_neu(Arc::new(Some(path.clone()).into_iter().collect()));
            db.set_input(path, Arc::new(input.into()));

            let diagnostics = db.all_diagnostics_sorted();
            if diagnostics.is_empty() {
                return "No errors".into();
            }
            diagnostics
                .into_iter()
                .map(|(path, range, diagnostic)| {
                    let path = db.lookup_file_id(path);
                    format!("{} | {:?} | {}", path.0, range, diagnostic)
                })
                .join("\n")
        })
        .unwrap();
    }

    #[test]
    fn md_errors_tests() {
        test_runner::test_snapshots("md", "errors", |input| {
            let mut db = TestDb::default();
            //let path: FileId = ("test.md".into(), FileKind::Md);
            let path = db.file_id(("test.md".into(), FileKind::Md));
            db.set_all_neu(Default::default());
            db.set_all_mds(Arc::new(Some(path.clone()).into_iter().collect()));
            db.set_input(path, Arc::new(input.into()));

            let diagnostics = db.all_diagnostics_sorted();
            if diagnostics.is_empty() {
                return "No errors".into();
            }

            diagnostics
                .into_iter()
                .map(|(path, range, diagnostic)| {
                    let path = db.lookup_file_id(path);
                    format!("{} | {:?} | {}", path.0, range, diagnostic)
                })
                .join("\n")
        })
        .unwrap();
    }
}
