use neu_diagnostics::Diagnostic;
use neu_eval::eval;
use neu_parser::NodeId;
use neu_render::db::Renderer;
use neu_syntax::db::{FileId, Parser};

#[salsa::query_group(DiagnosticianDatabase)]
pub trait Diagnostician: salsa::Database + Parser + Renderer {
    fn all_diagnostics(&self) -> Vec<(FileId, NodeId, Diagnostic)>;
}

fn all_diagnostics(db: &dyn Diagnostician) -> Vec<(FileId, NodeId, Diagnostic)> {
    let mut diagnostics: Vec<_> = db
        .parse_all_neu()
        .into_iter()
        .flat_map(|(path, id)| {
            let input = db.input_neu(path.clone());
            let mut parsed = db.parse_neu_syntax(path.clone());
            let _evaled = eval(id, &mut parsed.arena, &input);
            parsed
                .arena
                .components()
                .map(|(node_id, diagnostic)| (path.clone(), node_id, diagnostic.clone()))
                .collect::<Vec<_>>()
        })
        .collect();

    let md = db
        .parse_all_mds()
        .into_iter()
        .flat_map(|(_kind, _id, path, ast)| {
            let rendered = db.render_ast(path.clone(), ast);

            // render result contains ast from parser so it has both render and syntax errors.
            rendered
                .arena
                .components()
                .map(|(node_id, diagnostic)| (path.clone(), node_id, diagnostic.clone()))
                .collect::<Vec<_>>()
        });

    diagnostics.extend(md);
    diagnostics
}

#[salsa::database(
    neu_syntax::db::ParserDatabase,
    neu_render::db::RendererDatabase,
    neu_analyze::db::AnalyzerDatabase,
    crate::DiagnosticianDatabase
)]
#[derive(Default)]
pub struct Database {
    storage: salsa::Storage<Self>,
}
impl salsa::Database for Database {}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn neu_errors_tests() {
        test_runner::test_snapshots("neu", "errors", |input| {
            let mut db = Database::default();
            let path: String = "test.neu".into();
            db.set_all_mds(None.into_iter().collect());
            db.set_all_neu(Some(path.clone()).into_iter().collect());
            db.set_input_neu(path, input.into());

            let diagnostics = db.all_diagnostics();
            if diagnostics.is_empty() {
                return "No errors".into();
            }
            diagnostics
                .into_iter()
                .map(|(path, id, diagnostic)| format!("{} | {:?} | {}", path, id, diagnostic))
                .join("\n")
        })
        .unwrap();
    }

    #[test]
    fn md_errors_tests() {
        test_runner::test_snapshots("md", "errors", |input| {
            let mut db = Database::default();
            let path: String = "test.md".into();
            db.set_all_neu(None.into_iter().collect());
            db.set_all_mds(Some(path.clone()).into_iter().collect());
            db.set_input_md(path, input.into());

            let diagnostics = db.all_diagnostics();
            if diagnostics.is_empty() {
                return "No errors".into();
            }
            diagnostics
                .into_iter()
                .map(|(path, id, diagnostic)| format!("{} | {:?} | {}", path, id, diagnostic))
                .join("\n")
        })
        .unwrap();
    }
}
