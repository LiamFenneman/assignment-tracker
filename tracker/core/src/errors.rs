use thiserror::Error;

/// The value contained in the [mark](crate::prelude::Mark) is invalid.
#[derive(Error, Debug)]
pub enum MarkError {
    /// [`Percent`](crate::prelude::Mark::Percent) value is outside the valid range.
    #[error("value ({0}) is outside the valid range: 0.0 to 100.0")]
    Percent(f64),
    /// [`Letter`](crate::prelude::Mark::Letter) char is outside the valid range.
    #[error("char ({0}) is outside the valid range: A to Z")]
    Letter(char),
    /// [`OutOf`](crate::prelude::Mark::OutOf) left value is greater than right value.
    #[error("left value ({0}) is greater than right value ({1})")]
    OutOf(u32, u32),
}

/// The status is invalid.
#[derive(Error, Debug)]
pub enum StatusError {
    /// [Status](crate::prelude::Status) should be [`Marked`](crate::prelude::Status::Incomplete) when the [assignment mark](crate::prelude::Assignmentlike::mark) is set.
    #[error("assignment mark is set, status should be set to Marked")]
    NotMarked,
    /// [Status](crate::prelude::Status) should not be [`Marked`](crate::prelude::Status::Incomplete) when the [assignment mark](crate::prelude::Assignmentlike::mark) is `None`.
    #[error("assignment mark is None, status should not be set to Marked")]
    Marked,
}

/// The [assingment](crate::prelude::Assignment) is invalid.
#[derive(Error, Debug)]
pub enum AssignmentError {
    /// The `value` is invalid.
    #[error("value ({0}) must be within range 0.0 to 100.0")]
    Value(f64),
    /// The `mark` is invalid.
    #[error("mark is invalid: {0}")]
    Mark(#[from] MarkError),
    /// The `status` is invalid.
    #[error("status is invalid: {0}")]
    Status(#[from] StatusError),
}

/// The [tracker](crate::prelude::Trackerlike) is invalid.
#[derive(Error, Debug)]
pub enum TrackerError {
    /// Class code is already taken by another class.
    #[error("class code ({0}) already exists")]
    CodeTaken(String),
    /// Class code doesn't exist.
    #[error("could not find a class with code: {0}")]
    NoClass(String),
    /// Assignment ID is already taken by another assignment.
    #[error("assignment ID ({0}) already exists")]
    IdTaken(u32),
    /// Assignment ID doesn't exist.
    #[error("could not find a assignment with ID: {0}")]
    NoAssignment(u32),
    /// Assignment name is already taken by another assignment within the class.
    #[error("assignment name ({0}) already taken for {1}")]
    NameTaken(String, String),
    /// Invalid class.
    #[error("invalid class: {0}")]
    Class(#[from] ClassError),
}

/// The [class](crate::prelude::Classlike) is invalid.
#[derive(Error, Debug)]
pub enum ClassError {
    /// The total value of all assignments must be within `0.0..=100.0`
    #[error("total value ({0}) must be within 0.0 to 100.0")]
    TotalValue(f64),
}
