pub trait ToReport: Send + Sync {
    fn to_report(&self, str: &str) -> String;
    fn boxed(self) -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(self)
    }
}

pub type Diagnostic = Box<dyn ToReport>;

pub type Diagnostics = Vec<Diagnostic>;
