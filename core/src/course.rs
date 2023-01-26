use crate::Assignment;

/// Representation of a [Course].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Course {
    pub name: String,
    pub assignments: Vec<Assignment>,
}

impl Course {
    /// Create a new [Course] providing a name.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            ..Default::default()
        }
    }
}

impl Default for Course {
    fn default() -> Self {
        Self {
            name: String::from("Unknown course"),
            assignments: Vec::new(),
        }
    }
}
