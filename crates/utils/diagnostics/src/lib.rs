use std::collections::BTreeMap;
use std::fmt::Debug;

pub trait ToReport: Send + Sync {
    fn to_report(&self, str: &str) -> String;
    fn boxed(self) -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(self)
    }
}

pub type Diagnostic = String;

pub type DiagnosticVec = Vec<Diagnostic>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Diagnostics<NodeId>
where NodeId: Copy + Clone + PartialEq + Eq + PartialOrd + Ord + Debug
{
    errors: BTreeMap<NodeId, Diagnostic>
}

impl<NodeId> Default for Diagnostics<NodeId>
    where NodeId: Copy + Clone + PartialEq + Eq + PartialOrd + Ord + Debug
{
    fn default() -> Self {
        Self { errors: Default::default() }
    }
}

impl<NodeId> Diagnostics<NodeId>
where NodeId: Copy + Clone + PartialEq + Eq + PartialOrd + Ord + Debug
{
    pub fn add(&mut self, id: NodeId, c: Diagnostic) {
        self.errors.insert(id, c);
    }

    pub fn get(&self, id: NodeId) -> Option<&Diagnostic> {
        self.errors.get(&id)
    }

    pub fn iter(&self) -> impl Iterator<Item = (NodeId, &Diagnostic)> {
        self.errors.iter().map(|(id, e)| (*id, e))
    }

    pub fn merge(&mut self, other: &Self) {
        self.errors.extend(other.errors.clone().into_iter());
    }
}

impl<NodeId> IntoIterator for Diagnostics<NodeId>
    where NodeId: Copy + Clone + PartialEq + Eq + PartialOrd + Ord + Debug
{
    type Item = (NodeId, Diagnostic);
    type IntoIter = std::collections::btree_map::IntoIter<NodeId, Diagnostic>;

    fn into_iter(self) -> Self::IntoIter {
        self.errors.into_iter()
    }
}