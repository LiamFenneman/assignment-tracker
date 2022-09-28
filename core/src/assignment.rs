use time::Date;

use crate::mark::{Mark, Percent};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assignment {
    id: u64,
    course_id: u64,
    name: String,
    due_date: Option<Date>,
    weight: Option<Percent>,
    mark: Option<Mark>,
}

impl Assignment {
    /// Create a new assignment.
    #[must_use]
    pub fn new(id: u64, course_id: u64, name: String) -> Self {
        Assignment {
            id,
            course_id,
            name,
            due_date: None,
            weight: None,
            mark: None,
        }
    }

    /// Create a new assignment with a due date.
    #[must_use]
    pub fn with_due_date(mut self, due_date: Date) -> Self {
        self.due_date = Some(due_date);
        self
    }

    /// Create a new assignment with a weight.
    #[must_use]
    pub fn with_weight(mut self, weight: Percent) -> Self {
        self.weight = Some(weight);
        self
    }

    /// Create a new assignment with a mark.
    #[must_use]
    pub fn with_mark(mut self, mark: Mark) -> Self {
        self.mark = Some(mark);
        self
    }

    /// Get the assignment's ID.
    #[must_use]
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Get the assignment's course ID.
    #[must_use]
    pub fn course_id(&self) -> u64 {
        self.course_id
    }

    /// Get the assignment's name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the assignment's due date.
    #[must_use]
    pub fn due_date(&self) -> Option<Date> {
        self.due_date
    }

    /// Get the assignment's weight.
    #[must_use]
    pub fn weight(&self) -> Option<Percent> {
        self.weight
    }

    /// Get the assignment's mark.
    #[must_use]
    pub fn mark(&self) -> Option<Mark> {
        self.mark
    }
}
