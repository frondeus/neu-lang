use std::collections::HashMap;
use std::fmt::Debug;
use text_size::TextRange;

pub trait ToReport: Send + Sync {
    fn to_report(&self) -> String;
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
pub struct Diagnostics {
    errors: HashMap<TextRange, Diagnostic>
}

impl Default for Diagnostics {
    fn default() -> Self {
        Self { errors: Default::default() }
    }
}

impl Diagnostics {
    pub fn add(&mut self, range: TextRange, c: Diagnostic) {
        self.errors.insert(range, c);
    }

    pub fn get(&self, range: TextRange) -> Option<&Diagnostic> {
        self.errors.get(&range)
    }

    pub fn iter(&self) -> impl Iterator<Item = (TextRange, &Diagnostic)> {
        self.errors.iter().map(|(id, e)| (*id, e))
    }

    pub fn merge(&mut self, other: &Self) {
        self.errors.extend(other.errors.clone().into_iter());
    }
}

impl IntoIterator for Diagnostics
{
    type Item = (TextRange, Diagnostic);
    type IntoIter = std::collections::hash_map::IntoIter<TextRange, Diagnostic>;

    fn into_iter(self) -> Self::IntoIter {
        self.errors.into_iter()
    }
}
