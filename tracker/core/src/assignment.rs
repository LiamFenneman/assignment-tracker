mod mark;
mod status;

pub use mark::Mark;
pub use status::Status;

use crate::errors::InvalidAssignmentError;
use anyhow::Result;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};

/// Generic representation of an assignment.
pub trait Assignmentlike: Display + Debug + PartialEq + PartialOrd {
    /// The **unique** id of the [assignment](Assignmentlike).
    ///
    /// The **uniqueness** must be implemented on the **user** of an [assignment](Assignmentlike).
    fn id(&self) -> u32;

    /// The name of the [assignment](Assignmentlike).
    fn name(&self) -> &str;

    /// Represents how much the [assignment](Assignmentlike) is worth (by percentage) in relation to the other [assignments](Assignmentlike) in the [class](crate::prelude::Classlike).
    fn value(&self) -> Option<f64>;

    /// Set the value of the [assignment](Assignmentlike).
    fn set_value(&mut self, value: f64);

    /// Set the value of the [assignment](Assignmentlike) to `None`.
    fn remove_value(&mut self);

    /// The mark given for the [assignment](Assignmentlike).
    fn mark(&self) -> Option<Mark>;

    /// Set the mark of the [assignment](Assignmentlike) to a new value.
    ///
    /// # Errors
    /// - `mark` is invalid. See [`Mark::check_valid()`]
    fn set_mark(&mut self, mark: Mark) -> Result<()>;

    /// Set the mark of the [assignment](Assignmentlike) to `None`.
    fn remove_mark(&mut self);

    /// The due date of the [assignment](Assignmentlike).
    fn due_date(&self) -> Option<NaiveDateTime>;

    /// Set the due date of the [assignment](Assignmentlike).
    fn set_due_date(&mut self, due_date: NaiveDateTime);

    /// Set the due date of the [assignment](Assignmentlike) to `None`.
    fn remove_due_date(&mut self);

    /// The status of the [assignment](Assignmentlike).
    fn status(&self) -> Option<Status>;

    /// Set the status of the [assignment](Assignmentlike).
    fn set_status(&mut self, status: Status);

    /// Set the status of the [assignment](Assignmentlike) to `None`.
    fn remove_status(&mut self);
}

/// Basic implementation of [Assignmentlike].
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Assignment {
    id: u32,
    name: String,
    value: Option<f64>,
    mark: Option<Mark>,
    due_date: Option<NaiveDateTime>,
    status: Option<Status>,
}

impl Assignmentlike for Assignment {
    fn id(&self) -> u32 {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn value(&self) -> Option<f64> {
        self.value
    }

    fn set_value(&mut self, value: f64) {
        trace!("{self} -> Set value -> {value:?}");
        self.value = Some(value);
    }

    fn remove_value(&mut self) {
        trace!("{self} -> Set mark -> None");
        self.value = None;
    }

    fn mark(&self) -> Option<Mark> {
        self.mark
    }

    fn set_mark(&mut self, mark: Mark) -> Result<()> {
        mark.check_valid()?;
        trace!("{self} -> Set mark -> {mark:?}");
        self.mark = Some(mark);
        Ok(())
    }

    fn remove_mark(&mut self) {
        trace!("{self} -> Set mark -> None");
        self.mark = None;
    }

    fn due_date(&self) -> Option<NaiveDateTime> {
        self.due_date
    }

    fn set_due_date(&mut self, due_date: NaiveDateTime) {
        trace!("{self} -> Set due date -> {due_date:?}");
        self.due_date = Some(due_date);
    }

    fn remove_due_date(&mut self) {
        trace!("{self} -> Set due date -> None");
        self.due_date = None;
    }

    fn status(&self) -> Option<Status> {
        self.status
    }

    fn set_status(&mut self, status: Status) {
        trace!("{self} -> Set status -> {status:?}");
        self.status = Some(status);
    }

    fn remove_status(&mut self) {
        trace!("{self} -> Set status -> None");
        self.status = None;
    }
}

impl Default for Assignment {
    fn default() -> Self {
        Assignment::new(0, "Default Assignment")
    }
}

impl Assignment {
    /// Create a new [assignment](Assignment) with no value, mark, or due date.
    ///
    /// # Examples
    /// ```
    /// # use tracker_core::prelude::*;
    /// # use chrono::NaiveDate;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let assign = Assignment::new(0, "Test 1");
    ///
    /// let assign = Assignment::new(0, "Test 1")
    ///     .with_value(25.0)?
    ///     .with_mark(Mark::Letter('A'))?;
    ///
    /// let assign = Assignment::new(0, "Test 1")
    ///     .with_value(25.0)?
    ///     .with_due_date(NaiveDate::from_ymd(2022, 1, 1).and_hms(12, 0, 0));
    ///
    /// let assign = Assignment::new(0, "Test 1")
    ///     .with_value(25.0)?
    ///     .with_mark(Mark::Letter('A'))?
    ///     .with_due_date(NaiveDate::from_ymd(2022, 1, 1).and_hms(12, 0, 0));
    /// # Ok(()) }
    /// ```
    #[must_use]
    pub fn new(id: u32, name: &str) -> Self {
        Self {
            id,
            name: name.to_owned(),
            value: None,
            mark: None,
            due_date: None,
            status: None,
        }
    }

    /// Add a value to the [assignment](Assignment).
    ///
    /// # Errors
    /// - `value` not within range of 0.0 to 100.0.
    ///
    /// # Examples
    /// ```
    /// # use tracker_core::prelude::*;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut assign = Assignment::new(10, "Test 1")
    ///     .with_value(50.0)?;
    /// # Ok(()) }
    /// ```
    pub fn with_value(mut self, value: f64) -> Result<Self, InvalidAssignmentError> {
        // if value not in range [0, 100]
        if !(0.0..=100.0).contains(&value) {
            return Err(InvalidAssignmentError::Value(self.name, value));
        }
        self.value = Some(value);
        Ok(self)
    }

    /// Add a mark to the [assignment](Assignment).
    ///
    /// # Errors
    /// - `mark` is invalid. See [`Mark::check_valid()`]
    ///
    /// # Examples
    /// ```
    /// # use tracker_core::prelude::*;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut assign = Assignment::new(10, "Test 1")
    ///     .with_mark(Mark::Percent(75.0))?;
    /// # Ok(()) }
    /// ```
    pub fn with_mark(mut self, mark: Mark) -> Result<Self, InvalidAssignmentError> {
        if let Err(e) = mark.check_valid() {
            return Err(InvalidAssignmentError::Mark(self.name, e));
        }
        self.mark = Some(mark);
        Ok(self)
    }

    /// Add a due date to the [assignment](Assignment).
    ///
    /// # Examples
    /// ```
    /// # use tracker_core::prelude::*;
    /// # use chrono::NaiveDate;
    /// let mut assign = Assignment::new(10, "Test 1")
    ///     .with_due_date(NaiveDate::from_ymd(2022, 1, 1).and_hms(12, 0, 0));
    /// ```
    #[must_use]
    pub fn with_due_date(mut self, due_date: NaiveDateTime) -> Self {
        self.due_date = Some(due_date);
        self
    }

    /// Add a status to the [assignment](Assignment).
    ///
    /// # Examples
    /// ```
    /// # use tracker_core::prelude::*;
    /// let mut assign = Assignment::new(10, "Test 1")
    ///    .with_status(Status::Complete);
    /// ```
    #[must_use]
    pub fn with_status(mut self, status: Status) -> Self {
        self.status = Some(status);
        self
    }
}

impl Display for Assignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name(), self.id())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    mod set_mark {
        use super::*;

        #[rstest]
        #[case(Mark::Percent(0.0))]
        #[case(Mark::Percent(55.0))]
        #[case(Mark::Percent(100.0))]
        #[case(Mark::Letter('A'))]
        #[case(Mark::Letter('E'))]
        #[case(Mark::Letter('Z'))]
        #[case(Mark::OutOf(0, 0))]
        #[case(Mark::OutOf(0, 90))]
        #[case(Mark::OutOf(15, 25))]
        fn ok(#[case] mark: Mark) {
            let mut a = Assignment::default();
            let r = a.set_mark(mark);
            assert!(r.is_ok(), "{mark:?}");
        }

        #[rstest]
        #[case(Mark::Percent(-10.0))]
        #[case(Mark::Percent(155.0))]
        #[case(Mark::Letter('a'))]
        #[case(Mark::Letter('#'))]
        #[case(Mark::Letter(' '))]
        #[case(Mark::Letter('1'))]
        #[case(Mark::OutOf(1, 0))]
        #[case(Mark::OutOf(15, 12))]
        fn err(#[case] mark: Mark) {
            let mut a = Assignment::default();
            let r = a.set_mark(mark);
            assert!(r.is_err(), "{mark:?}");
        }
    }
}
