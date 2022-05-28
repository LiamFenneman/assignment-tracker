use crate::{err, Assignment, Assignmentlike, Class, Classlike};
use anyhow::Result;
use std::fmt::{Debug, Display};

/// Keep track of many [assignments](crate::Assignmentlike) from many [classes](crate::Classlike).
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tracker<C = Class, A = Assignment>
where
    C: Classlike,
    A: Assignmentlike,
{
    name: String,
    classes: Vec<C>,
    assignments: Vec<A>,
}

impl<C, A> Tracker<C, A>
where
    C: Classlike,
    A: Assignmentlike,
{
    /// Create a new [tracker][Tracker] with a given name.
    ///
    /// # Examples
    /// ```
    /// # use tracker_core::*;
    /// let tracker = Tracker::<Class>::new("Class Tracker");
    /// let tracker = Tracker::<Code>::new("Code Tracker");
    /// ```
    #[must_use]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            classes: Vec::new(),
            assignments: Vec::new(),
        }
    }

    /// Get all the classes within the [tracker](Tracker).
    #[must_use]
    pub fn get_classes(&self) -> &[C] {
        &self.classes
    }

    /// Get all the assignments within the [tracker](Tracker).
    #[must_use]
    pub fn get_assignments(&self) -> &[A] {
        &self.assignments
    }

    /// Add a [class](crate::Classlike) to the [tracker](Tracker).
    ///
    /// # Errors
    /// - `class.code()` already taken
    pub fn add_class(&mut self, class: C) -> Result<()> {
        if self.get_classes().iter().any(|c| c.code() == class.code()) {
            let code = class.code();
            err!("{self} -> Class code ({code}) already exists");
        }

        trace!("{self} -> Add class -> {class:?}");
        self.classes.push(class);
        Ok(())
    }

    /// Remove a [class](crate::Classlike) from the [tracker](Tracker).
    ///
    /// # Errors
    /// - `code` isn't used by any [class](Classlike)
    pub fn remove_class(&mut self, code: &str) -> Result<C> {
        let Some(index) = self.get_classes().iter().position(|c| c.code() == code) else {
            err!("{self} -> Could not find a class with code: {code}");
        };

        let c = self.classes.remove(index);
        trace!("{self} -> Remove class -> {c:?}");
        Ok(c)
    }

    pub fn add_assignment(&mut self, _code: &str, _assign: A) -> Result<()> {
        todo!()
    }

    pub fn remove_assignment(&mut self, _assign_id: u32) -> Result<A> {
        todo!()
    }
}

impl<C, A> Display for Tracker<C, A>
where
    C: Classlike,
    A: Assignmentlike,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl<C, A> Default for Tracker<C, A>
where
    C: Classlike,
    A: Assignmentlike,
{
    fn default() -> Self {
        Self {
            name: String::from("Default Tracker"),
            classes: Vec::new(),
            assignments: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    use rstest::rstest;

    #[rstest]
    #[case("TEST123")]
    #[case("OTHER456")]
    #[case("ANOTHER")]
    #[case("MyClassCode")]
    #[case("Test123")]
    fn add_class(#[case] s: &str) {
        let mut t = Tracker::<Code>::default();
        assert!(t.add_class(Code::new(s)).is_ok());

        // double add
        assert!(t.add_class(Code::new(s)).is_err());
    }

    #[rstest]
    #[case("TEST123")]
    #[case("OTHER456")]
    #[case("ANOTHER")]
    #[case("MyClassCode")]
    #[case("Test123")]
    fn remove_class(#[case] s: &str) {
        let mut t = Tracker::<Code>::default();
        assert!(t.add_class(Code::new(s)).is_ok());
        assert!(t.remove_class(s).is_ok());

        // double remove
        assert!(t.remove_class(s).is_err());
    }

    #[rstest]
    #[case("Test 1")]
    #[case("Assignment 4")]
    #[case("Exam")]
    #[case("MY ASSESSMENT")]
    #[case("ASSESSMENT")]
    fn add_assignment(#[case] s: &str) {
        let mut t = Tracker::<Code>::default();
        assert!(t.add_class(Code::new("TEST123")).is_ok());
        assert!(t
            .add_assignment("TEST123", Assignment::new(1, s, 15.0))
            .is_ok());

        // double add
        assert!(t
            .add_assignment("TEST123", Assignment::new(1, s, 25.0))
            .is_err());
    }

    #[rstest]
    #[case("Test 1")]
    #[case("Assignment 4")]
    #[case("Exam")]
    #[case("MY ASSESSMENT")]
    #[case("ASSESSMENT")]
    fn remove_assignment(#[case] s: &str) {
        let mut t = Tracker::<Code>::default();
        assert!(t.add_class(Code::new("TEST123")).is_ok());
        assert!(t
            .add_assignment("TEST123", Assignment::new(1, s, 15.0))
            .is_ok());
        assert!(t.remove_assignment(1).is_ok());

        // double add
        assert!(t.remove_assignment(1).is_err());
    }
}
