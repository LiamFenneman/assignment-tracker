use crate::{err, Mark};
use anyhow::Result;
use chrono::NaiveDateTime;
use std::fmt::Display;

/// Generic representation of an assignment.
pub trait Assignmentlike: Display + PartialEq {
    /// The **unique** id of the [assignment](Assignmentlike).
    ///
    /// The **uniqueness** must be implemented on the **user** of an [assignment](Assignmentlike).
    fn id(&self) -> u32;

    /// The name of the [assignment](Assignmentlike).
    fn name(&self) -> &str;

    /// Represents how much the [assignment](Assignmentlike) is worth in relation to the other [assignments](Assignmentlike) in the [class](crate::Classlike).
    fn value(&self) -> f64;

    /// The mark given for the [assignment](Assignmentlike).
    fn mark(&self) -> Option<Mark>;

    /// The due date of the [assignment](Assignmentlike).
    fn due_date(&self) -> Option<NaiveDateTime>;

    /// Set the mark of the [assignment](Assignmentlike) to a new value.
    ///
    /// # Errors
    /// - `mark` is invalid. See [`Mark::is_valid()`]
    fn set_mark(&mut self, mark: Mark) -> Result<()>;

    /// Set the mark to `None`.
    fn remove_mark(&mut self);

    /// Set the due date of the [assignment](Assignmentlike).
    fn set_due_date(&mut self, due_date: NaiveDateTime);

    /// Set the due date to `None`.
    fn remove_due_date(&mut self);
}

/// Basic implementation of [Assignmentlike].
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Assignment {
    id: u32,
    name: String,
    value: f64,
    mark: Option<Mark>,
    due_date: Option<NaiveDateTime>,
}

impl Assignmentlike for Assignment {
    fn id(&self) -> u32 {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn value(&self) -> f64 {
        self.value
    }

    fn mark(&self) -> Option<Mark> {
        self.mark
    }

    fn due_date(&self) -> Option<NaiveDateTime> {
        self.due_date
    }

    fn set_mark(&mut self, mark: Mark) -> Result<()> {
        if !mark.is_valid() {
            err!("Provided mark is invalid: {mark:?}");
        }

        self.mark = Some(mark);
        Ok(())
    }

    fn remove_mark(&mut self) {
        self.mark = None;
    }

    fn set_due_date(&mut self, due_date: NaiveDateTime) {
        self.due_date = Some(due_date);
    }

    fn remove_due_date(&mut self) {
        self.due_date = None;
    }
}

impl Display for Assignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name(), self.id())
    }
}
