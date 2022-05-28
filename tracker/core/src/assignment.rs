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

        trace!("{self} -> Set mark -> {mark:?}");
        self.mark = Some(mark);
        Ok(())
    }

    fn remove_mark(&mut self) {
        trace!("{self} -> Set mark -> None");
        self.mark = None;
    }

    fn set_due_date(&mut self, due_date: NaiveDateTime) {
        trace!("{self} -> Set due date -> {due_date:?}");
        self.due_date = Some(due_date);
    }

    fn remove_due_date(&mut self) {
        trace!("{self} -> Set due date -> None");
        self.due_date = None;
    }
}

impl Default for Assignment {
    fn default() -> Self {
        Assignment::new(0, "Default Assignment", 0.0)
    }
}

impl Assignment {
    /// Create a new [assignment](Assignment) with no mark or due date.
    ///
    /// # Examples
    /// ```
    /// # use tracker_core::*;
    /// let assign = Assignment::new(10, "Test 1", 35.0);
    /// let assign = Assignment::new(25, "Assignment 1", 15.0);
    /// let assign = Assignment::new(102, "Exam", 55.0);
    /// ```
    #[must_use]
    pub fn new(id: u32, name: &str, value: f64) -> Self {
        Self {
            id,
            name: name.to_owned(),
            value,
            mark: None,
            due_date: None,
        }
    }

    /// Create a new [assignment](Assignment) using the builder pattern.
    ///
    /// # Examples
    /// ```
    /// # use tracker_core::*;
    /// # use chrono::NaiveDate;
    /// let assign = Assignment::builder(10, "Test 1", 35.0).build();
    /// assert_eq!(Assignment::new(10, "Test 1", 35.0), assign);
    ///
    /// let assign = Assignment::builder(5, "Exam", 35.0)
    ///     .mark(Mark::Percent(75.0))
    ///     .build();
    ///
    /// let assign = Assignment::builder(5, "Exam", 35.0)
    ///     .due_date(NaiveDate::from_ymd(2022, 05, 01).and_hms(12, 33, 15))
    ///     .build();
    ///
    /// let assign = Assignment::builder(5, "Exam", 35.0)
    ///     .due_date(NaiveDate::from_ymd(2022, 05, 01).and_hms(12, 33, 15))
    ///     .mark(Mark::Letter('A'))
    ///     .build();
    /// ```
    #[must_use]
    pub fn builder(id: u32, name: &str, value: f64) -> Builder {
        Builder {
            id,
            name: name.to_owned(),
            value,
            mark: None,
            due_date: None,
        }
    }
}

/// Builder for [Assignment].
pub struct Builder {
    id: u32,
    name: String,
    value: f64,
    mark: Option<Mark>,
    due_date: Option<NaiveDateTime>,
}

impl Builder {
    /// Get the built [assignment](Assignment).
    pub fn build(&mut self) -> Assignment {
        let a = Assignment {
            id: self.id,
            name: self.name.clone(),
            value: self.value,
            mark: self.mark,
            due_date: self.due_date,
        };
        trace!("Assignment built -> {a:?}");
        a
    }

    /// Add the mark to the [assignment](Assignment).
    pub fn mark(&mut self, mark: Mark) -> &mut Self {
        self.mark = Some(mark);
        self
    }

    /// Add the due date to the [assignment](Assignment).
    pub fn due_date(&mut self, due_date: NaiveDateTime) -> &mut Self {
        self.due_date = Some(due_date);
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
    use chrono::NaiveDate;
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

    #[rstest]
    #[case(None, None)]
    #[case(Some(Mark::Percent(75.0)), None)]
    #[case(None, Some(NaiveDate::from_ymd(2022, 05, 01).and_hms(12, 25, 30)))]
    #[case(Some(Mark::Percent(50.0)), Some(NaiveDate::from_ymd(2022, 12, 25).and_hms(14, 45, 10)))]
    fn builder(#[case] mark: Option<Mark>, #[case] due_date: Option<NaiveDateTime>) {
        let func = |id: u32| {
            let name = format!("Assignment {id}");
            let mut ass = Assignment::builder(id, &name, 0.0);
            if mark.is_none() && due_date.is_none() {
                let ass = ass.build();
                assert_eq!(
                    Assignment {
                        id,
                        name,
                        value: 0.0,
                        mark: None,
                        due_date: None
                    },
                    ass
                );
            } else if mark.is_none() {
                let ass = ass.due_date(due_date.unwrap().clone()).build();
                assert_eq!(
                    Assignment {
                        id,
                        name,
                        value: 0.0,
                        mark: None,
                        due_date
                    },
                    ass
                );
            } else if due_date.is_none() {
                let ass = ass.mark(mark.unwrap()).build();
                assert_eq!(
                    Assignment {
                        id,
                        name,
                        value: 0.0,
                        mark,
                        due_date: None
                    },
                    ass
                );
            } else {
                let ass = ass.mark(mark.unwrap()).due_date(due_date.unwrap()).build();
                assert_eq!(
                    Assignment {
                        id,
                        name,
                        value: 0.0,
                        mark,
                        due_date
                    },
                    ass
                );
            }
        };

        for i in 0..10 {
            func(i);
        }
    }
}
