use crate::errors::InvalidTrackerError::{
    AssignmentIdNone, AssignmentIdTaken, AssignmentNameNotUnique, ClassCodeNone, ClassCodeTaken,
};
use crate::prelude::*;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
};

/// Definition of methods required to be able to be a tracker.
pub trait Trackerlike<'de, C, A>: Serialize + Deserialize<'de>
where
    C: Classlike + Serialize + Deserialize<'de>,
    A: Assignmentlike + Serialize + Deserialize<'de>,
{
    /// Get the name of the tracker.
    ///
    /// This *should* be unique, but is **not** enforced.
    #[must_use]
    fn name(&self) -> &str;

    /// Create a new [tracker][Tracker] with a given name.
    ///
    /// # Examples
    /// ```
    /// # use tracker_core::prelude::*;
    /// let tracker = Tracker::<Class>::new("Class Tracker");
    /// let tracker = Tracker::<Code>::new("Code Tracker");
    /// ```
    #[must_use]
    fn new(name: &str) -> Self;

    /// Get all the classes within the [tracker](Trackerlike).
    #[must_use]
    fn classes(&self) -> &[C];

    /// Get all the classes within the [tracker](Trackerlike).
    #[must_use]
    fn classes_mut(&mut self) -> &mut [C];

    /// Get all the assignments within the [tracker](Trackerlike).
    #[must_use]
    fn assignments(&self) -> &[A];

    /// Get all the assignments within the [tracker](Trackerlike).
    #[must_use]
    fn assignments_mut(&mut self) -> &mut [A];

    /// Get all the assignments within the [tracker](Trackerlike) that belong to a [class](Classlike) with the provided `code`.
    #[must_use]
    fn assignments_from_class(&self, code: &str) -> Vec<&A>;

    /// Get all the assignments within the [tracker](Trackerlike) that belong to a [class](Classlike) with the provided `code`.
    #[must_use]
    fn assignments_from_class_mut(&mut self, code: &str) -> Vec<&mut A>;

    /// Add a [class](Classlike) to the [tracker](Trackerlike).
    ///
    /// # Errors
    /// - `class.code()` already taken
    fn add_class(&mut self, class: C) -> Result<()>;

    /// Remove a [class](Classlike) from the [tracker](Trackerlike).
    ///
    /// # Errors
    /// - `code` isn't used by any [class](Classlike)
    fn remove_class(&mut self, code: &str) -> Result<C>;

    /// Add an [assignment](Assignmentlike) to the [tracker](Trackerlike).
    ///
    /// # Errors
    /// - `assign.id()` already taken by another assignment
    /// - `assign.name()` already taken within the class
    /// - No class with `code` exists
    /// - Total value of assignments within `code` greater than `100.0`
    fn add_assignment(&mut self, code: &str, assign: A) -> Result<()>;

    /// Remove an [assignment](Assignmentlike) from the [tracker](Trackerlike).
    ///
    /// # Errors
    /// - No assignment with id: `assign_id` exists within the tracker
    fn remove_assignment(&mut self, assign_id: u32) -> Result<A>;

    /// Get the [assignment](Assignmentlike) that corresponds to the given *ID*.
    fn get_assignment(&self, id: u32) -> Option<&A> {
        self.assignments().iter().find(|a| a.id() == id)
    }

    /// Get the [assignment](Assignmentlike) that corresponds to the given *ID*.
    fn get_assignment_mut(&mut self, id: u32) -> Option<&mut A> {
        self.assignments_mut().iter_mut().find(|a| a.id() == id)
    }

    /// Get the [class](Classlike) that corresponds to the given *code*.
    fn get_class(&self, code: &str) -> Option<&C> {
        self.classes().iter().find(|c| c.code() == code)
    }

    /// Get mutable reference to the [class](Classlike) that corresponds to the given *code*.
    fn get_class_mut(&mut self, code: &str) -> Option<&mut C> {
        self.classes_mut().iter_mut().find(|c| c.code() == code)
    }
}

/// Keep track of many [assignments](Assignmentlike) from many [classes](Classlike).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Tracker<C = Class, A = Assignment>
where
    C: Classlike,
    A: Assignmentlike,
{
    name: String,
    classes: Vec<C>,
    assignments: Vec<A>,
    map: HashMap<u32, String>,
}

impl<'de, C, A> Trackerlike<'de, C, A> for Tracker<C, A>
where
    C: Classlike + Serialize + Deserialize<'de>,
    A: Assignmentlike + Serialize + Deserialize<'de>,
{
    fn name(&self) -> &str {
        &self.name
    }

    fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            classes: Vec::new(),
            assignments: Vec::new(),
            map: HashMap::new(),
        }
    }

    fn classes(&self) -> &[C] {
        &self.classes
    }

    fn classes_mut(&mut self) -> &mut [C] {
        &mut self.classes
    }

    fn assignments(&self) -> &[A] {
        &self.assignments
    }

    fn assignments_mut(&mut self) -> &mut [A] {
        &mut self.assignments
    }

    fn assignments_from_class(&self, code: &str) -> Vec<&A> {
        let ids = self
            .map
            .iter()
            .filter(|(_, c)| *c == code)
            .map(|(id, _)| *id)
            .collect::<Vec<_>>();
        self.assignments
            .iter()
            .filter(|a| ids.contains(&a.id()))
            .collect::<Vec<_>>()
    }

    fn assignments_from_class_mut(&mut self, code: &str) -> Vec<&mut A> {
        let ids = self
            .map
            .iter()
            .filter(|(_, c)| *c == code)
            .map(|(id, _)| *id)
            .collect::<Vec<_>>();
        self.assignments
            .iter_mut()
            .filter(|a| ids.contains(&a.id()))
            .collect::<Vec<_>>()
    }

    fn add_class(&mut self, class: C) -> Result<()> {
        if self.classes().iter().any(|c| c.code() == class.code()) {
            bail!(ClassCodeTaken(
                self.name().to_owned(),
                class.code().to_owned()
            ));
        }

        trace!("{self} -> Add class -> {class:?}");

        // add the class to the vec
        self.classes.push(class);
        Ok(())
    }

    fn remove_class(&mut self, code: &str) -> Result<C> {
        let Some(index) = self.classes().iter().position(|c| c.code() == code) else {
            bail!(ClassCodeTaken(
                self.name().to_owned(),
                code.to_owned()
            ));
        };

        let ids = self
            .map
            .iter()
            .filter(|&(_, c)| c == code)
            .map(|(&id, _)| id)
            .collect::<Vec<u32>>();

        ids.iter().for_each(|id| drop(self.map.remove(id)));

        // remove the class from the vec
        let c = self.classes.remove(index);

        trace!("{self} -> Remove class -> {c:?}");
        Ok(c)
    }

    fn add_assignment(&mut self, code: &str, assign: A) -> Result<()> {
        if self.assignments().iter().any(|a| a.id() == assign.id()) {
            bail!(AssignmentIdTaken(self.name().to_owned(), assign.id()));
        }

        // ensure unique assignment name within a class
        if self
            .assignments
            .iter()
            .filter(|&a| a.name() == assign.name())
            .map(Assignmentlike::id)
            .any(|id| self.map.get(&id).is_some_and(|&s| s == code))
        {
            bail!(AssignmentNameNotUnique(
                self.name().to_owned(),
                assign.name().to_owned(),
                code.to_owned()
            ));
        }

        // ensure total value within class is less than 100
        match self.get_class_mut(code) {
            None => {
                bail!(ClassCodeNone(self.name().to_owned(), code.to_owned()));
            }
            Some(class) => {
                class.add_total_value(assign.value().unwrap_or(0.0))?;
            }
        };

        // insert entry (assign id -> class code) into the map
        self.map.insert(assign.id(), code.to_owned());

        trace!("{self} -> Add assignment -> {assign:?}");

        // add the assignment to the vec
        self.assignments.push(assign);
        Ok(())
    }

    fn remove_assignment(&mut self, assign_id: u32) -> Result<A> {
        let Some(index) = self.assignments().iter().position(|a| a.id() == assign_id) else {
            bail!( AssignmentIdNone(self.name().to_owned(), assign_id));
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

impl<'de, C, A> Default for Tracker<C, A>
where
    C: Classlike + Serialize + Deserialize<'de>,
    A: Assignmentlike + Serialize + Deserialize<'de>,
{
    fn default() -> Self {
        Self::new("Default Tracker")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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

    mod assignment_from_class {
        use super::*;

        fn setup(a: u32, b: u32) -> Tracker<Code> {
            let mut t = Tracker::<Code>::default();
            assert!(t.add_class(Code::new("CLASS A")).is_ok());
            assert!(t.add_class(Code::new("CLASS B")).is_ok());
            let gen = |a, b: u32| Assignment::new(a, &format!("A{}", b));
            for i in 0..a {
                assert!(t.add_assignment("CLASS A", gen(i, i)).is_ok());
            }
            for i in 0..b {
                assert!(t.add_assignment("CLASS B", gen(i + a, i)).is_ok());
            }
            t
        }

        #[rstest]
        #[case(0, 0)]
        #[case(1, 0)]
        #[case(2, 3)]
        #[case(4, 4)]
        #[case(5, 7)]
        fn ok(#[case] a: u32, #[case] b: u32) {
            let t = setup(a, b);

            let va = t.assignments_from_class("CLASS A");
            assert_eq!(va.len(), a as usize);

            let vb = t.assignments_from_class("CLASS B");
            assert_eq!(vb.len(), b as usize);
        }
    }

    mod add_assignment {
        use super::*;

        #[rstest]
        #[case("Test 1")]
        #[case("Assignment 4")]
        #[case("Exam")]
        #[case("MY ASSESSMENT")]
        #[case("ASSESSMENT")]
        fn same_id(#[case] s: &str) {
            let mut t = Tracker::<Code>::default();
            assert!(t.add_class(Code::new("TEST123")).is_ok());
            assert!(t.add_assignment("TEST123", Assignment::new(1, s)).is_ok());

            // one entry in the map
            assert_eq!(1, t.map.len());

            // double add
            assert!(t.add_assignment("TEST123", Assignment::new(1, s)).is_err());
        }

        #[rstest]
        #[case("Test 1")]
        #[case("Assignment 4")]
        #[case("Exam")]
        fn same_name(#[case] name: &str) {
            let mut t = Tracker::<Code>::default();
            assert!(t.add_class(Code::default()).is_ok());
            let code = "DEFAULT";
            assert!(t.add_assignment(code, Assignment::new(0, name)).is_ok());
            assert!(t.add_assignment(code, Assignment::new(1, name)).is_err());
        }

        #[rstest]
        #[case("Test 1")]
        #[case("Assignment 4")]
        #[case("Exam")]
        fn no_class(#[case] name: &str) {
            let mut t = Tracker::<Code>::default();
            assert!(t.add_assignment("Class", Assignment::new(0, name)).is_err());
        }

        mod class_total_value {
            use super::*;

            #[rstest]
            #[case(25.0, 75.0)]
            #[case(50.0, 50.0)]
            #[case(75.0, 25.0)]
            #[case(99.9, 0.1)]
            fn ok(#[case] a1: f64, #[case] a2: f64) {
                let mut t = Tracker::<Code>::default();
                assert!(t.add_class(Code::default()).is_ok());
                let code = "DEFAULT";
                assert!(t
                    .add_assignment(code, Assignment::new(0, "Test 1").with_value(a1))
                    .is_ok());
                assert!(t
                    .add_assignment(code, Assignment::new(1, "Test 2").with_value(a2))
                    .is_ok());
            }

            #[rstest]
            #[case(25.0, 100.0)]
            #[case(50.0, 55.0)]
            #[case(75.0, 30.0)]
            #[case(100.0, 0.1)]
            fn err(#[case] a1: f64, #[case] a2: f64) {
                let mut t = Tracker::<Code>::default();
                assert!(t.add_class(Code::default()).is_ok());
                let code = "DEFAULT";
                assert!(t
                    .add_assignment(code, Assignment::new(0, "Test 1").with_value(a1))
                    .is_ok());
                assert!(t
                    .add_assignment(code, Assignment::new(1, "Test 2").with_value(a2))
                    .is_err());
            }
        }
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
        assert!(t.add_assignment("TEST123", Assignment::new(1, s)).is_ok());
        assert!(t.remove_assignment(1).is_ok());

        // double add
        assert!(t.remove_assignment(1).is_err());
    }

    mod serde {
        use super::*;

        #[test]
        fn default() {
            let tracker = Tracker::<Code>::default();
            let expect = tracker.clone();

            let json = serde_json::to_string(&tracker);
            assert!(json.is_ok());

            let str = json.unwrap();
            println!("{str}");

            let de = serde_json::from_str::<Tracker<Code>>(&str);
            assert!(de.is_ok());
            assert_eq!(de.unwrap(), expect);
        }

        #[test]
        fn many() {
            const CLASS_A: &str = "CLASS 111";
            const CLASS_B: &str = "OTHER 999";
            const N: u32 = 3;

            let gen = |a, b| Assignment::new(a, &format!("Assign {b}"));

            let mut tracker = Tracker::<Code>::default();
            tracker.add_class(Code::new(CLASS_A)).unwrap();
            tracker.add_class(Code::new(CLASS_B)).unwrap();
            for i in 0..N {
                tracker.add_assignment(CLASS_A, gen(i, i)).unwrap();
                tracker.add_assignment(CLASS_B, gen(i + N, i)).unwrap();
            }
            let expect = tracker.clone();

            let json = serde_json::to_string_pretty(&tracker);
            assert!(json.is_ok());

            let str = json.unwrap();
            println!("{str}");

            let de = serde_json::from_str::<Tracker<Code>>(&str);
            assert!(de.is_ok());
            assert_eq!(de.unwrap(), expect);
        }
    }
}
