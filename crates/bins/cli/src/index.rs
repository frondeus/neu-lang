use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Debug, Serialize)]
#[allow(dead_code)] // TODO: Finish me
enum Tree {
    Dir(String, Vec<Tree>),
    File(usize),
    None,
}

#[derive(Debug, Serialize)]
pub struct Index {
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

#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
pub struct IndexEntry {
    pub kind: String,
    pub id: String,
    pub title: String,
    pub path: String,
}
