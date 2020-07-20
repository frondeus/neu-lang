use neu_canceled::Canceled;
use neu_diagnostics::Diagnostic;
use neu_parser::NodeId;
use neu_render::db::Renderer;
use neu_syntax::db::{FileId, Parser};

#[salsa::query_group(DiagnosticianDatabase)]
pub trait Diagnostician: salsa::Database + Parser + Renderer {
    fn all_diagnostics(&self) -> Vec<(FileId, NodeId, Diagnostic)>;
}

fn all_diagnostics(db: &dyn Diagnostician) -> Vec<(FileId, NodeId, Diagnostic)> {
    Canceled::cancel_if(db.salsa_runtime());

    let mut diagnostics: Vec<_> = db
        .parse_all_neu()
        .into_iter()
        .flat_map(|(path, id)| {
            // eva result contains ast from parser so it has both eval and syntax errors.
            let evaled = db.eval(path, id);
            evaled
                .arena
                .components()
                .map(|(node_id, diagnostic)| (path, node_id, diagnostic.clone()))
                .collect::<Vec<_>>()
        })
        .collect();

    let md = db
        .parse_all_mds()
        .into_iter()
        .flat_map(|(_kind, _id, path, ast)| {
            let rendered = db.render_ast(path, ast);

            // render result contains ast from parser so it has both render and syntax errors.
            rendered
                .arena
                .components()
                .map(|(node_id, diagnostic)| (path, node_id, diagnostic.clone()))
                .collect::<Vec<_>>()
        });

    diagnostics.extend(md);
    diagnostics
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

            let diagnostics = db.all_diagnostics();
            if diagnostics.is_empty() {
                return "No errors".into();
            }
            diagnostics
                .into_iter()
                .map(|(path, id, diagnostic)| {
                    let path = db.lookup_file_id(path);
                    format!("{} | {:?} | {}", path.0, id, diagnostic)
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

            let diagnostics = db.all_diagnostics();
            if diagnostics.is_empty() {
                return "No errors".into();
            }
            diagnostics
                .into_iter()
                .map(|(path, id, diagnostic)| {
                    let path = db.lookup_file_id(path);
                    format!("{} | {:?} | {}", path.0, id, diagnostic)
                })
                .join("\n")
        })
        .unwrap();
    }
}
