use crate::assignment::Assignment;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Course {
    id: u64,
    short_name: String,
    long_name: String,
    assignments: Vec<Assignment>,
}

impl Course {
    /// Create a new course.
    #[must_use]
    pub fn new(id: u64, short_name: String, long_name: String) -> Self {
        Course {
            id,
            short_name,
            long_name,
            assignments: Vec::new(),
        }
    }

    /// Get the course's ID.
    #[must_use]
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Get the course's short name.
    #[must_use]
    pub fn short_name(&self) -> &str {
        &self.short_name
    }

    /// Get the course's long name.
    #[must_use]
    pub fn long_name(&self) -> &str {
        &self.long_name
    }

    /// Get the course's assignments.
    #[must_use]
    pub fn assignments(&self) -> &[Assignment] {
        &self.assignments
    }

    /// Add an assignment to the course.
    pub fn add_assignment(&mut self, assignment: Assignment) {
        self.assignments.push(assignment);
    }

    /// Remove an assignment from the course.
    pub fn remove_assignment(&mut self, id: u64) {
        self.assignments.retain(|a| a.id() != id);
    }

    /// Get an assignment from the course.
    #[must_use]
    pub fn assignment_by_id(&self, id: u64) -> Option<&Assignment> {
        self.assignments.iter().find(|a| a.id() == id)
    }

    /// Get an assignment from the course.
    #[must_use]
    pub fn assignment_by_id_mut(&mut self, id: u64) -> Option<&mut Assignment> {
        self.assignments.iter_mut().find(|a| a.id() == id)
    }

    /// Get an assignment from the course.
    #[must_use]
    pub fn assignment_by_name(&self, name: &str) -> Option<&Assignment> {
        self.assignments.iter().find(|a| a.name() == name)
    }

    /// Get an assignment from the course.
    #[must_use]
    pub fn assignment_by_name_mut(&mut self, name: &str) -> Option<&mut Assignment> {
        self.assignments.iter_mut().find(|a| a.name() == name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(1, "CS 101", "Introduction to Computer Science")]
    #[case(2, "CS 102", "Data Structures and Algorithms")]
    #[case(3, "CS 103", "Operating Systems")]
    fn course_new(#[case] id: u64, #[case] short_name: &str, #[case] long_name: &str) {
        let course = Course::new(id, short_name.to_string(), long_name.to_string());
        assert_eq!(course.id(), id);
        assert_eq!(course.short_name(), short_name);
        assert_eq!(course.long_name(), long_name);
    }
}
