use serde::{Deserialize, Serialize};

/// Status enum with two values: `Incomplete` and `Complete`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Status {
    /// Assignment is incomplete.
    Incomplete,
    /// Assignment is complete.
    Complete,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Incomplete => write!(f, "Incomplete"),
            Status::Complete => write!(f, "Complete"),
        }
    }
}
