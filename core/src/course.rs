#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Course {
    id: u64,
    short_name: String,
    long_name: String,
}

impl Course {
    /// Create a new course.
    #[must_use]
    pub fn new(id: u64, short_name: String, long_name: String) -> Self {
        Course {
            id,
            short_name,
            long_name,
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
