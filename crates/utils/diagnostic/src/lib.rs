use std::fmt;

#[derive(Clone, Copy)]
pub struct NodeId(pub(crate) usize);

impl NodeId {
    pub fn new(id: usize) -> Self { Self(id) }
    pub fn id(&self) -> usize { self.0 }
}

impl fmt::Debug for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "N{}", self.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Severity {
    Error,
    Warning,
    Hint
}

#[derive(Debug)]
pub struct Error {
    severity: Severity,
    error: Box<dyn ToReport>,
    location: NodeId,
    context: Vec<NodeId>,
}

impl Error {
    pub fn new<E: ToReport + 'static>(severity: Severity, error: E, location: NodeId, context: Vec<NodeId>) -> Self {
        Self {
            severity,
            location,
            context,
            error: Box::new(error)
        }
    }

    pub fn severity(&self) -> Severity {
        self.severity
    }

    pub fn location(&self) -> NodeId {
        self.location
    }

    pub fn context(&self) -> &[NodeId] {
        &self.context
    }
}

pub trait ToReport: fmt::Debug {
    fn to_report<T>(self) -> Report<T> where Self: Sized;
}

pub struct Report<T> {
    desc: String,
    locations: Vec<Location<T>>,
    severity: Severity
}

impl<T> Report<T> {
    pub fn severity(&self) -> Severity {
        self.severity
    }

    pub fn desc(&self) -> &str {
        &self.desc
    }

    pub fn locations(&self) -> &[Location<T>] {
        &self.locations
    }
}

pub struct Location<T> {
    line: (T, T),
    col: (T, T),
}

impl<T> Location<T> 
where
    T: Clone + Copy
{
    pub fn line(&self) -> T {
        self.line.0
    }

    pub fn end_line(&self) -> T {
        self.line.1
    }

    pub fn col(&self) -> T {
        self.col.0
    }

    pub fn end_col(&self) -> T {
        self.col.1
    }
}

