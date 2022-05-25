use crate::{err, MAX_NAME_LEN};
use anyhow::{bail, Result};
use std::fmt::Display;

/// A single assignment.
#[derive(Debug, Clone, PartialEq)]
pub struct Assignment {
    id: u64,
    name: String,
    mark: Option<f64>,
    value: f64,
}

impl Assignment {
    /// Create a new [assignment](Assignment) providing an ID, name and value.
    ///
    /// # Errors
    /// - `name` is empty
    /// - `name` is longer than [`MAX_NAME_LEN`] (in bytes)
    /// - `value` is not within the range `0.0..=100.0`
    ///
    /// # Example
    /// ```
    /// # use tracker_core::Assignment;
    /// let assign = Assignment::new(0, "Assignment 1", 25.0);
    /// assert!(assign.is_ok());
    /// ```
    pub fn new(id: u64, name: &str, value: f64) -> Result<Self> {
        if name.is_empty() {
            err!("An assignment name must be provied.");
        }

        if name.len() > MAX_NAME_LEN {
            err!("Assignment name ({name}) is too long - must be below {MAX_NAME_LEN} bytes.");
        }

        if !(0.0..=100.0).contains(&value) {
            err!("The value of an assignment must be within 0.0 and 100.0 -> provided: {value}");
        }

        let a = Self {
            id,
            name: name.to_owned(),
            mark: None,
            value,
        };

        trace!("Created assignment: {a:?}");
        Ok(a)
    }

    /// Create a new [assignment](Assignment) providing an ID, name, value and mark.
    ///
    /// # Errors
    /// - Propagates errors from [`Assignment::new()`]
    /// - Propagates errors from [`Assignment::set_mark()`]
    ///
    /// # Example
    /// ```
    /// # use tracker_core::Assignment;
    /// let assign = Assignment::new_with_mark(0, "Assignment 1", 25.0, 25.0);
    /// assert!(assign.is_ok());
    /// ```
    pub fn new_with_mark(id: u64, name: &str, value: f64, mark: f64) -> Result<Self> {
        let mut a = Self::new(id, name, value)?;
        a.set_mark(mark)?;
        Ok(a)
    }

    /// Set the mark of this [assignment](Assignment).
    ///
    /// # Errors
    /// - `mark` not within the range `0.0..=100.0`
    ///
    /// # Example
    /// ```
    /// # use anyhow::Result;
    /// # use tracker_core::Assignment;
    /// # fn main() -> Result<()> {
    /// let mut assign = Assignment::new(0, "Assignment 1", 25.0)?;
    /// assign.set_mark(75.0);
    ///
    /// assert!(assign.mark().is_some());
    /// if let Some(mark) = assign.mark() {
    ///     assert_eq!(75.0, mark);
    /// }
    /// # Ok(()) }
    /// ```
    pub fn set_mark(&mut self, mark: f64) -> Result<()> {
        if !(0.0..=100.0).contains(&mark) {
            err!("Mark must be within 0.0 and 100.0 -> provided: {mark}");
        }

        info!("{self} -> Set mark to {mark}");
        self.mark = Some(mark);
        Ok(())
    }

    /// Remove the mark from this [assignment](Assignment).
    ///
    /// # Example
    /// ```
    /// # use anyhow::Result;
    /// # use tracker_core::Assignment;
    /// # fn main() -> Result<()> {
    /// let mut assign = Assignment::new_with_mark(0, "Assignment 1", 25.0, 75.0)?;
    /// # assert_eq!(75.0, assign.mark().unwrap());
    /// assign.remove_mark();
    ///
    /// assert!(assign.mark().is_none());
    /// # Ok(()) }
    /// ```
    pub fn remove_mark(&mut self) {
        match self.mark {
            Some(_) => info!("{self} -> Set mark to None"),
            None => warn!("{self} -> Attempt to remove mark which is already set to None"),
        }
        self.mark = None;
    }

    /// Get the ID for this [assignment](Assignment).
    #[must_use]
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Get a reference to the name of this [assignment](Assignment).
    #[must_use]
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Get the mark for this [assignment](Assignment).
    #[must_use]
    pub fn mark(&self) -> Option<f64> {
        self.mark
    }

    /// Get the value of this [assignment](Assignment).
    #[must_use]
    pub fn value(&self) -> f64 {
        self.value
    }
}

impl Display for Assignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (id, name) = (self.id, &self.name);
        write!(f, "{name} [ID: {id}]")
    }
}

impl PartialOrd for Assignment {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    mod new {
        use super::*;

        #[rstest]
        #[case("Assignment 1", 30.0)]
        #[case("Assignment 2", 0.0)]
        #[case("Assignment 3", 100.0)]
        fn ok(#[case] name: &str, #[case] value: f64) {
            let a = Assignment::new(0, name, value);

            assert!(a.is_ok());
            assert_eq!(
                Assignment {
                    id: 0,
                    name: name.to_owned(),
                    mark: None,
                    value,
                },
                a.unwrap()
            );
        }

        #[rstest]
        #[case("", 30.0)]
        #[case("really long assignment name which is way too long", 30.0)]
        #[case("Assignment 1", -1.0)]
        #[case("Assignment 2", 101.0)]
        fn err(#[case] name: &str, #[case] value: f64) {
            let a = Assignment::new(0, name, value);
            assert!(a.is_err());
        }
    }

    mod new_with_mark {
        use super::*;

        #[rstest]
        #[case("Assignment 1", 30.0, 80.0)]
        #[case("Assignment 2", 0.0, 100.0)]
        #[case("Assignment 3", 100.0, 0.0)]
        fn ok(#[case] name: &str, #[case] value: f64, #[case] mark: f64) {
            let a = Assignment::new_with_mark(0, name, value, mark);

            assert!(a.is_ok());
            assert_eq!(
                Assignment {
                    id: 0,
                    name: name.to_owned(),
                    mark: Some(mark),
                    value,
                },
                a.unwrap()
            );
        }

        #[rstest]
        #[case("Assignment 1", 30.0, -1.0)]
        #[case("Assignment 2", 0.0, 101.0)]
        fn err(#[case] name: &str, #[case] value: f64, #[case] mark: f64) {
            let a = Assignment::new_with_mark(0, name, value, mark);
            assert!(a.is_err());
        }
    }

    #[rstest]
    #[case("Assignment 1", Some(90.0), 30.0)]
    #[case("Assignment 2", None, 30.0)]
    fn display(#[case] name: &str, #[case] mark: Option<f64>, #[case] value: f64) {
        let a = Assignment {
            id: 0,
            name: name.to_owned(),
            mark,
            value,
        };
        println!("{a}");
        assert!(format!("{a}").contains(name));
    }

    mod set_mark {
        use super::*;

        #[rstest]
        #[case("Assignment 1", 90.0, 30.0)]
        #[case("Assignment 2", 75.0, 30.0)]
        fn ok(#[case] name: &str, #[case] mark: f64, #[case] value: f64) {
            let mut a = Assignment::new(0, name, value).unwrap();

            let res = a.set_mark(mark);
            assert!(res.is_ok());
            assert_eq!(
                Assignment {
                    id: 0,
                    name: name.to_owned(),
                    mark: Some(mark),
                    value,
                },
                a
            );
        }

        #[rstest]
        #[case("Assignment 1", 190.0, 30.0)]
        #[case("Assignment 2", -75.0, 30.0)]
        fn err(#[case] name: &str, #[case] mark: f64, #[case] value: f64) {
            let mut a = Assignment::new(0, name, value).unwrap();

            let res = a.set_mark(mark);
            assert!(res.is_err());
            println!("{res:?}");
        }
    }
}
