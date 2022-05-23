use crate::{err, Assignment, Class};
use anyhow::{bail, Result};
use log::{error, info};
use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Clone)]
pub struct Tracker {
    name: String,
    classes: HashMap<u64, Class>,
}

impl Tracker {
    /// Create a new [Tracker].
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            classes: HashMap::new(),
        }
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

    pub fn track_assignment(&mut self, _cid: u64, _assign: Assignment) -> Result<()> {
        todo!()
    }
}

impl Display for Tracker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Default for Tracker {
    fn default() -> Self {
        Self {
            name: "Tracker".to_owned(),
            classes: Default::default(),
        }
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
        //use super::*;
    }
}
