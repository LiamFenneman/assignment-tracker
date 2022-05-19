use crate::{err, Assignment};
use anyhow::{bail, Result};
use log::error;
use std::fmt::Display;

/// Representation of a generic class or university paper.
#[derive(Debug, Clone, PartialEq)]
pub struct Class {
    id: u64,
    short_name: String,
    assignments: Vec<Assignment>,
    total_value: f64,
}

impl Class {
    /// Create a new [Class] providing an ID and short name.
    pub fn new(id: u64, short_name: &str) -> Self {
        let short_name = short_name.to_owned();
        Class {
            id,
            short_name,
            assignments: Vec::new(),
            total_value: 0.0,
        }
    }

    /// Add a new [assignment](Assignment) to the [class](Class).
    ///
    /// # Constraints
    /// - `total_value + assign.value() > 100.0`
    /// - `assignments` already contains `assign`
    /// - An assignment in the class already has the same ID
    pub fn add_assignment(&mut self, assign: Assignment) -> Result<()> {
        let total = self.total_value + assign.value();
        if total > 100.0 {
            err!("Could not add {assign} due to the total value of the class exceeds 100.0");
        }

        if self.assignments.iter().any(|a| *a == assign) {
            err!("Could not add {assign} due to it already existing within the class.");
        }

        if self.assignments.iter().any(|a| a.id() == assign.id()) {
            let id = assign.id();
            err!("Could not add {assign} due to the ID ({id}) already existing within the class.");
        }

        if self.assignments.iter().any(|a| a.name() == assign.name()) {
            let name = assign.name();
            err!(
                "Could not add {assign} due to the name ({name}) already existing within the class."
            );
        }

        self.total_value = total;
        self.assignments.push(assign);
        Ok(())
    }

    /// Get the ID for this [class](Class).
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Get the short name for this [class](Class).
    pub fn short_name(&self) -> &String {
        &self.short_name
    }

    /// Get a reference to the list of [assignments](Assignment) for this [class](Class).
    pub fn assignments(&self) -> &Vec<Assignment> {
        &self.assignments
    }

    /// Get the total value of all the [assignments](Assignment).
    pub fn total_value(&self) -> f64 {
        self.total_value
    }
}

impl IntoIterator for Class {
    type Item = Assignment;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.assignments.into_iter()
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (id, sn) = (self.id, &self.short_name);
        write!(f, "{sn} [ID: {id}]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn display() {
        let name = "TEST101";
        let c = Class::new(0, name);
        println!("{c}");
        assert!(format!("{c}").contains(name));
    }

    mod add_assignment {
        use super::*;

        #[rstest]
        #[case(90.0, 15.0, true)]
        #[case(15.0, 90.0, true)]
        #[case(50.0, 50.0, false)]
        fn total_value(#[case] v1: f64, #[case] v2: f64, #[case] is_err: bool) {
            let mut class = Class::new(0, "TEST101");

            let a = Assignment::builder(0).name("Test 0").value(v1).build();
            let res = class.add_assignment(a);
            assert!(res.is_ok());

            let a = Assignment::builder(1).name("Test 1").value(v2).build();
            let res = class.add_assignment(a);
            assert_eq!(is_err, res.is_err());

            if let Err(e) = res {
                println!("{e}");
                return;
            }

            assert_eq!(v1 + v2, class.total_value());
        }

        #[rstest]
        #[case((0, "Test 1", 50.0), (0, "Test 1", 50.0))] // Equal Assignments
        #[case((0, "Test 1", 50.0), (0, "Test 2", 50.0))] // Same ID
        #[case((0, "Test 1", 50.0), (1, "Test 1", 40.0))] // Same Name
        fn constraints(#[case] t1: (u64, &str, f64), #[case] t2: (u64, &str, f64)) {
            let mut class = Class::new(0, "TEST101");
            let a = Assignment::builder(t1.0).name(t1.1).value(t1.2).build();
            let b = Assignment::builder(t2.0).name(t2.1).value(t2.2).build();

            let res = class.add_assignment(a);
            assert!(res.is_ok());

            let res = class.add_assignment(b);
            assert!(res.is_err());
            println!("{res:?}");
        }
    }
}
