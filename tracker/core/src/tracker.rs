use crate::{err, Assignment, Class};
use anyhow::{bail, Result};
use log::{error, info, trace};
use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Clone)]
pub struct Tracker {
    name: String,
    classes: HashMap<u64, Class>,
}

impl Tracker {
    /// Create a new [Tracker].
    pub fn new(name: &str) -> Self {
        let t = Self {
            name: name.to_owned(),
            classes: HashMap::new(),
        };
        trace!("Created tracker: {t:?}");
        t
    }

    /// Add a new [class](Class) to the [tracker](Tracker).
    ///
    /// # Constraints
    /// - `classes` already contains `class`
    /// - A [class](Class) in the [tracker](Tracker) already has the same ID
    /// - A [class](Class) in the [tracker](Tracker) already has the same name
    pub fn track_class(&mut self, class: Class) -> Result<()> {
        if class.short_name().is_empty() {
            err!("Could not add {class} -> No short name was provided.");
        }

        if self.classes.iter().any(|(id, _)| *id == class.id()) {
            let id = class.id();
            err!("Could not add {class} -> ID ({id}) already exists.");
        }

        if self
            .classes
            .iter()
            .any(|(_, c)| c.short_name() == class.short_name())
        {
            let name = class.short_name();
            err!("Could not add {class} -> Name ({name}) already exists.");
        }

        info!("Added {class} to {self}.");
        self.classes.insert(class.id(), class);
        Ok(())
    }

    /// Add a new [assignment](Assignment) to the [class](Class) within this [tracker](Tracker).
    pub fn track_assignment(&mut self, cid: u64, assign: Assignment) -> Result<()> {
        let Some(class) = self.classes.get_mut(&cid) else {
            err!("Could not find the class with ID: {cid}");
        };

        class.add_assignment(assign)?;
        Ok(())
    }
}

impl Display for Tracker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Default for Tracker {
    fn default() -> Self {
        Self::new("Tracker")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    mod track_class {
        use super::*;

        #[rstest]
        #[case(Class::new(0, "TEST123"))]
        #[case(Class::new(1, "TEST456"))]
        #[case(Class::new(999, "Class"))]
        fn ok(#[case] c: Class) {
            let mut t = Tracker::default();
            let r = t.track_class(c);
            assert!(r.is_ok());
            assert_eq!(1, t.classes.len());
        }

        #[rstest]
        #[case(
            Class::new(0, "TEST123"),
            Class::new(0, "TEST456"),
            "Classes have the same ID -- should return Err"
        )]
        #[case(
            Class::new(0, "TEST123"),
            Class::new(1, "TEST123"),
            "Classes have the same short name -- should return Err"
        )]
        #[case(
            Class::new(0, "TEST123"),
            Class::new(1, ""),
            "No short name was provided - should return Err"
        )]
        fn err(#[case] c1: Class, #[case] c2: Class, #[case] msg: &str) {
            let mut t = Tracker::default();
            let r1 = t.track_class(c1);
            assert!(r1.is_ok());

            let r2 = t.track_class(c2);
            println!("{r2:?}");
            assert!(r2.is_err(), "{msg}");
        }
    }

    mod track_assignment {
        use super::*;

        #[test]
        fn ok() {
            let mut tracker = Tracker::default();
            assert!(tracker.track_class(Class::new(0, "TEST123")).is_ok());
            let a1 = Assignment::new(0, "Test 1", 50.0).unwrap();
            let a2 = Assignment::new(1, "Test 2", 50.0).unwrap();
            assert!(tracker.track_assignment(0, a1).is_ok());
            assert!(tracker.track_assignment(0, a2).is_ok());
        }

        #[rstest]
        #[case(0, (0, "Test 2", 50.0))]
        #[case(0, (1, "Test 1", 50.0))]
        #[case(0, (1, "Test 2", 100.0))]
        #[case(1, (1, "Test 2", 10.0))]
        fn err(#[case] cid: u64, #[case] a2: (u64, &str, f64)) {
            let mut tracker = Tracker::default();
            assert!(tracker.track_class(Class::new(0, "TEST123")).is_ok());
            let a1 = Assignment::new(0, "Test 1", 50.0).unwrap();
            let a2 = Assignment::new(a2.0, a2.1, a2.2).unwrap();
            assert!(tracker.track_assignment(0, a1).is_ok());
            assert!(tracker.track_assignment(cid, a2).is_err());
        }
    }
}
