use serde::{Deserialize, Serialize};

/// [Assignment](crate::prelude::Assignmentlike) status: `Incomplete`, `Complete`, or `Marked`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Status {
    /// Assignment is incomplete.
    Incomplete,
    /// Assignment is complete and mark is pending.
    Complete,
    /// Assignment is complete and marked.
    Marked,
}

impl Default for Status {
    fn default() -> Self {
        Status::Incomplete
    }
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Incomplete => write!(f, "Incomplete"),
            Status::Complete => write!(f, "Complete"),
            Status::Marked => write!(f, "Marked"),
        }
    }
}
