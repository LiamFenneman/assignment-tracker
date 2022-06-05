use thiserror::Error;

/// The value contained in the [mark](crate::prelude::Mark) is invalid.
#[derive(Error, Debug)]
pub enum InvalidMarkError {
    /// [`Percent`](crate::prelude::Mark::Percent) value is outside the valid range
    #[error("Mark::Percent -> value ({0}) is outside the valid range: 0.0 to 100.0")]
    Percent(f64),
    /// [`Letter`](crate::prelude::Mark::Letter) char is outside the valid range
    #[error("Mark::Letter -> char ({0}) is outside the valid range: A to Z")]
    Letter(char),
    /// [`OutOf`](crate::prelude::Mark::OutOf) left value is greater than right value
    #[error("Mark::OutOf -> left value ({0}) is greater than right value ({1})")]
    OutOf(u32, u32),
}

/// The [assingment](crate::prelude::Assignment) is invalid.
#[derive(Error, Debug)]
pub enum InvalidAssignmentError {
    /// The `value` is invalid.
    #[error("{0} -> value ({1}) must be within range 0.0..=100.0")]
    Value(String, f64),
    /// The `mark` is invalid.
    #[error("{0} -> {1}")]
    Mark(String, InvalidMarkError),
}

/// The [tracker](crate::prelude::Trackerlike) is invalid.
#[derive(Error, Debug)]
pub enum InvalidTrackerError {
    /// Class code is already taken by another class.
    #[error("{0} -> Class code ({1}) already exists")]
    CodeTaken(String, String),
    /// Class code doesn't exist.
    #[error("{0} -> Could not find a class with code: {1}")]
    NoClass(String, String),
    /// Assignment ID is already taken by another assignment.
    #[error("{0} -> Assignment ID ({1}) already exists")]
    IdTaken(String, u32),
    /// Assignment ID doesn't exist.
    #[error("{0} -> Could not find a assignment with ID: {1}")]
    NoAssignment(String, u32),
    /// Assignment name is already taken by another assignment within the class.
    #[error("{0} -> Assignment name ({1}) already taken for {2}")]
    NameTaken(String, String, String),
}

/// The [class](crate::prelude::Classlike) is invalid.
#[derive(Error, Debug)]
pub enum InvalidClassError {
    /// The total value of all assignments must be within `0.0..=100.0`
    #[error("{0} -> Total value ({1}) must be within 0.0..=100.0")]
    TotalValue(String, f64),
}
