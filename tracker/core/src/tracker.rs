use crate::{err, Assignment, Assignmentlike, Class, Classlike};
use anyhow::Result;
use std::{
    collections::BTreeMap,
    fmt::{Debug, Display},
};

pub trait Trackerlike<C, A>
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
    fn new(name: &str) -> Self;

    /// Get all the classes within the [tracker](Trackerlike).
    #[must_use]
    fn get_classes(&self) -> &[C];

    /// Get all the assignments within the [tracker](Trackerlike).
    #[must_use]
    fn get_assignments(&self) -> &[A];

    /// Add a [class](crate::Classlike) to the [tracker](Trackerlike).
    ///
    /// # Errors
    /// - `class.code()` already taken
    fn add_class(&mut self, class: C) -> Result<()>;

    /// Remove a [class](crate::Classlike) from the [tracker](Trackerlike).
    ///
    /// # Errors
    /// - `code` isn't used by any [class](Classlike)
    fn remove_class(&mut self, code: &str) -> Result<C>;

    fn add_assignment(&mut self, code: &str, assign: A) -> Result<()>;

    fn remove_assignment(&mut self, assign_id: u32) -> Result<A>;

    /// Get the [assignment](Assignmentlike) that corresponds to the given *ID*.
    fn get_assignment_by_id(&self, id: u32) -> Option<&A> {
        self.get_assignments().iter().find(|a| a.id() == id)
    }

    /// Get the [class](Classlike) that corresponds to the given *code*.
    fn get_class_by_code(&self, code: &str) -> Option<&C> {
        self.get_classes().iter().find(|c| c.code() == code)
    }
}

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
    map: BTreeMap<u32, String>,
}

impl<C, A> Trackerlike<C, A> for Tracker<C, A>
where
    C: Classlike,
    A: Assignmentlike,
{
    fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            classes: Vec::new(),
            assignments: Vec::new(),
            map: BTreeMap::new(),
        }
    }

    fn get_classes(&self) -> &[C] {
        &self.classes
    }

    fn get_assignments(&self) -> &[A] {
        &self.assignments
    }

    fn add_class(&mut self, class: C) -> Result<()> {
        if self.get_classes().iter().any(|c| c.code() == class.code()) {
            let code = class.code();
            err!("{self} -> Class code ({code}) already exists");
        }

        trace!("{self} -> Add class -> {class:?}");

        // add the class to the vec
        self.classes.push(class);
        Ok(())
    }

    fn remove_class(&mut self, code: &str) -> Result<C> {
        let Some(index) = self.get_classes().iter().position(|c| c.code() == code) else {
            err!("{self} -> Could not find a class with code: {code}");
        };

        // TODO: remove all entries in the map for class code

        // remove the class from the vec
        let c = self.classes.remove(index);

        trace!("{self} -> Remove class -> {c:?}");
        Ok(c)
    }

    fn add_assignment(&mut self, code: &str, assign: A) -> Result<()> {
        if self.get_assignments().iter().any(|a| a.id() == assign.id()) {
            let id = assign.id();
            err!("{self} -> Assignment ID ({id}) already exists.");
        }

        // TODO: ensure unique assignment name for class

        // insert entry (assign id -> class code) into the map
        self.map.insert(assign.id(), code.to_owned());

        trace!("{self} -> Add assignment -> {assign:?}");

        // add the assignment to the vec
        self.assignments.push(assign);
        Ok(())
    }

    fn remove_assignment(&mut self, assign_id: u32) -> Result<A> {
        let Some(index) = self.get_assignments().iter().position(|a| a.id() == assign_id) else {
            err!("{self} -> Could not find a assignment with ID: {assign_id}");
        };

        // remove the entry in map
        self.map.remove(&assign_id);

        // remove the class from the vec
        let a = self.assignments.remove(index);

        trace!("{self} -> Remove assignment -> {a:?}");
        Ok(a)
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
            map: BTreeMap::new(),
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
