use std::sync::Arc;
use std::hash::Hash;
use crate::log_writer::LogWriter;

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum Destination {
    None,
    StdOut,
    StdErr,
    File(String),
    Custom(Box<dyn CustomDestination>),
}

pub trait CustomDestination: Send + Sync {

    fn normalized(&self) -> Destination;
    fn as_writer(&self) -> Arc<dyn LogWriter + Send + Sync>;
    fn clone_box(&self) -> Box<dyn CustomDestination>;

    // This should be a unique identifier
    fn name(&self) -> String;
}

impl Clone for Box<dyn CustomDestination> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl PartialEq for Box<dyn CustomDestination> {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}

impl Eq for Box<dyn CustomDestination> {}

impl Hash for Box<dyn CustomDestination> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name().hash(state);
    }
}