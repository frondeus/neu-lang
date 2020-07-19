use crate::Mention;
use neu_canceled::Canceled;
use neu_syntax::db::Parser;

#[salsa::query_group(AnalyzerDatabase)]
pub trait Analyzer: salsa::Database + Parser {
    fn all_mentions(&self) -> Vec<Mention>;
}

fn all_mentions(db: &dyn Analyzer) -> Vec<Mention> {
    Canceled::cancel_if(db.salsa_runtime());

    let parsed = db.parse_all_mds();

    let mut mentions = vec![];
    for (_kind, _id, path, article_item) in parsed {
        let input = db.input(path.clone());
        let mut parsed = db.parse_syntax(path);
        let _ = crate::find_mentions(db, article_item, &mut parsed.arena, &input, &mut mentions);
    }

    mentions
}
