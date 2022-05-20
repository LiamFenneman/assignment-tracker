use crate::{err, Assignment};
use anyhow::{bail, Result};
use log::{error, info};
use std::{collections::HashMap, fmt::Display};

/// Representation of a generic class or university paper.
#[derive(Debug, Clone, PartialEq)]
pub struct Class {
    id: u64,
    short_name: String,
    assignments: HashMap<u64, Assignment>,
    total_value: f64,
}

impl Class {
    /// Create a new [Class] providing an ID and short name.
    pub fn new(id: u64, short_name: &str) -> Self {
        let short_name = short_name.to_owned();
        Class {
            id,
            short_name,
            assignments: HashMap::new(),
            total_value: 0.0,
        }
    }

    /// Add a new [assignment](Assignment) to the [class](Class).
    ///
    /// # Constraints
    /// - `total_value + assign.value() > 100.0`
    /// - An [assignment](Assignment) in the [class](Class) already has the same ID
    /// - An [assignment](Assignment) in the [class](Class) already has the same name
    pub fn add_assignment(&mut self, assign: Assignment) -> Result<()> {
        let total = self.total_value + assign.value();
        if total > 100.0 {
            err!("Could not add {assign} -> Total value of the class exceeds 100.0");
        }

        if self.assignments.iter().any(|(id, _)| *id == assign.id()) {
            let id = assign.id();
            err!("Could not add {assign} -> ID ({id}) already exists.");
        }

        if self
            .assignments
            .iter()
            .any(|(_, a)| a.name() == assign.name())
        {
            let name = assign.name();
            err!("Could not add {assign} -> Name ({name}) already exists.");
        }

        info!("Added {assign} to {self}. Total value now: {total}");
        self.total_value = total;
        self.assignments.insert(assign.id(), assign);
        Ok(())
    }

    /// Remove an [assignment](Assignment) from the [class](Class) which has the given ID.
    pub fn remove_assignment(&mut self, id: u64) -> Result<Assignment> {
        match self.assignments.remove(&id) {
            Some(a) => {
                info!("Removed {a} from {self}");
                Ok(a)
            }
            None => {
                err!("Could not find assignment ID: {id} in {self}.");
            }
        }
    }

    /// Add the mark to an [assignment](Assignment) with the provided ID.
    ///
    /// # Constraints
    /// - `mark` is within range: `0.0..=100.0`
    pub fn add_mark(&mut self, id: u64, mark: f64) -> Result<()> {
        if !(0.0..=100.0).contains(&mark) {
            err!("The given mark ({mark}) is outside the valid range (0.0..=100.0).");
        }

        match self.assignments.get_mut(&id) {
            Some(a) => a.set_mark(mark)?,
            None => {
                err!("Could not find assignment ID: {id} in {self}.");
            }
        }

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
    pub fn assignments(&self) -> &HashMap<u64, Assignment> {
        &self.assignments
    }

    /// Get the total value of all the [assignments](Assignment).
    pub fn total_value(&self) -> f64 {
        self.total_value
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
        #[case(None)]
        #[case(Some(80.0))]
        fn valid(#[case] mark: Option<f64>) {
            let mut class = Class::new(0, "TEST101");
            let res = class.add_assignment(
                Assignment::builder(0)
                    .name("Test 1")
                    .value(50.0)
                    .mark(mark)
                    .build(),
            );
            assert!(res.is_ok());
            assert_eq!(1, class.assignments().len());
        }

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

    mod remove_assignment {
        use super::*;

        #[test]
        fn valid() {
            let mut class = Class::new(0, "TEST101");
            let _ = class.add_assignment(
                Assignment::builder(0)
                    .name("Test 1")
                    .value(50.0)
                    .mark(Some(80.0))
                    .build(),
            );
            let _ = class.add_assignment(Assignment::builder(1).name("Test 2").value(50.0).build());
            assert_eq!(2, class.assignments().len());

            let res = class.remove_assignment(1);
            assert!(res.is_ok());
            assert_eq!(1, class.assignments().len());
        }
    }

    mod add_mark {
        use super::*;

        #[rstest]
        #[case(0.0)]
        #[case(50.0)]
        #[case(100.0)]
        fn ok(#[case] mark: f64) -> Result<()> {
            let mut class = Class::new(0, "TEST101");
            class.add_assignment(Assignment::builder(0).name("Test 1").value(50.0).build())?;

            let res = class.add_mark(0, mark);
            assert!(res.is_ok());
            assert!(class.assignments()[&0].mark().is_some());
            assert_eq!(mark, class.assignments()[&0].mark().unwrap());

            Ok(())
        }

        #[rstest]
        #[case(-1.0)]
        #[case(101.0)]
        fn err(#[case] mark: f64) -> Result<()> {
            let mut class = Class::new(0, "TEST101");
            class.add_assignment(Assignment::builder(0).name("Test 1").value(50.0).build())?;

            let res = class.add_mark(0, mark);
            println!("{res:?}");
            assert!(res.is_err());

            Ok(())
        }
    }
}
