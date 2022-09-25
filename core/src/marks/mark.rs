use crate::mark::{out_of, percent, Grade, OutOf, Percent};

/// A mark for an assignment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mark {
    Percent(Percent),
    Grade(Grade),
    OutOf(OutOf),
}

impl Mark {
    /// Create a new percent mark.
    ///
    /// # Errors
    /// - If the `value` is greater than 100.
    pub fn percent(value: u8) -> Result<Self, percent::Error> {
        Ok(Mark::Percent(Percent::new(value)?))
    }

    /// Create a new letter grade.
    #[must_use]
    pub fn letter(grade: Grade) -> Self {
        Mark::Grade(grade)
    }

    /// Create a new out of mark.
    ///
    /// # Errors
    /// - If `mark` is greater than `out_of`.
    pub fn out_of(mark: u16, out_of: u16) -> Result<Self, out_of::Error> {
        Ok(Mark::OutOf(OutOf::new(mark, out_of)?))
    }
}
