#[derive(Default, Clone)]
pub struct HashCount {
    pub(crate) count: usize,
}

impl From<()> for HashCount {
    fn from(_: ()) -> Self {
                                 Default::default()
                                                   }
}

impl From<HashCount> for () {
    fn from(_: HashCount) -> Self {}
}
