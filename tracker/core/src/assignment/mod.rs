mod invalid_error;
pub use invalid_error::InvalidError;

use regex::Regex;
use std::{cmp, fmt, rc::Rc, result};

use crate::ClassCode;

/// Representation of a single assignment.
#[derive(PartialEq, Debug, Clone)]
pub struct Assignment {
    name: String,
    mark: Option<f64>,
    value: f64,
    percent: Option<f64>,
    class_code: Rc<ClassCode>,
}

pub type Result<T> = result::Result<T, InvalidError>;

// use lazy static to create the regex once
lazy_static! {
    static ref RE: Regex = Regex::new(r"^[A-Z]{4}\d{3}$").unwrap();
}

impl Assignment {
    /// Create a new [`Assignment`].
    ///
    /// # Conditions
    /// - `name` length within range `3..=20`
    /// - `value` within range `0..=100`
    ///
    /// # Examples
    /// ```
    /// # use std::rc::Rc;
    /// # use tracker_core::{Assignment, ClassCode};
    /// let code = Rc::new(ClassCode::new("SOME101")?);
    /// let assign = Assignment::new("Test", 10.0, code);
    /// assert!(assign.is_ok());
    /// # Ok::<(), &'static str>(())
    /// ```
    ///
    /// ***Note:*** *using [`Rc`] allows for using the same instance of [`ClassCode`]*
    /// ```
    /// # use std::rc::Rc;
    /// # use tracker_core::{Assignment, ClassCode};
    /// let code = Rc::new(ClassCode::new("SOME101")?);
    /// let assign1 = Assignment::new("Test 1", 10.0, Rc::clone(&code));
    /// let assign2 = Assignment::new("Test 2", 10.0, Rc::clone(&code));
    /// assert!(assign1.is_ok());
    /// assert!(assign2.is_ok());
    /// # Ok::<(), &'static str>(())
    /// ```
    pub fn new(name: &str, value: f64, class_code: Rc<ClassCode>) -> Result<Self> {
        let ass = Self {
            name: name.to_string(),
            mark: None,
            value,
            percent: None,
            class_code,
        };

        if let Err(e) = ass.is_valid() {
            return Err(e);
        }

        Ok(ass)
    }

    /// Get the name of the [assignment](Assignment).
    ///
    /// # Example
    /// ```
    /// # use std::rc::Rc;
    /// # use tracker_core::{Assignment, ClassCode};
    /// let code = Rc::new(ClassCode::new("SOME101")?);
    /// let assign = Assignment::new("Test", 10.0, code)?;
    /// assert_eq!("Test", assign.name());
    /// # Ok::<(), &'static str>(())
    /// ```
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Get the mark for the [assignment](Assignment).
    ///
    /// # Example
    /// ```
    /// # use std::rc::Rc;
    /// # use tracker_core::{Assignment, ClassCode};
    /// let code = Rc::new(ClassCode::new("SOME101")?);
    /// let mut assign = Assignment::new("Test", 10.0, code)?;
    ///
    /// /// Mark is `None` by default.
    /// assert_eq!(None, assign.mark());
    ///
    /// assign.set_mark(90.0);
    /// assert_eq!(90.0, assign.mark().unwrap());
    /// # Ok::<(), &'static str>(())
    /// ```
    pub fn mark(&self) -> Option<f64> {
        self.mark
    }

    /// Set the mark for the [assignment](Assignment).
    ///
    /// # Conditions
    /// If the mark is not `None` then both `m >= 0.0` and `m <= 100.0` must hold for the mark to be set.
    ///
    /// # Examples
    /// ```
    /// # use std::rc::Rc;
    /// # use tracker_core::{Assignment, ClassCode};
    /// let code = Rc::new(ClassCode::new("SOME101")?);
    /// let mut assign = Assignment::new("Test", 10.0, code)?;
    ///
    /// /// Valid mark
    /// assert!(assign.set_mark(0.01).is_ok());
    /// assert!(assign.set_mark(80.0).is_ok());
    /// assert!(assign.set_mark(100.0).is_ok());
    ///
    /// /// Invalid marks
    /// assert!(assign.set_mark(-1.0).is_err());
    /// assert!(assign.set_mark(0.0).is_err());
    /// assert!(assign.set_mark(101.0).is_err());
    /// # Ok::<(), &'static str>(())
    /// ```
    pub fn set_mark(&mut self, mark: f64) -> Result<()> {
        if mark <= 0.0 {
            return Err(InvalidError("Mark must be positive"));
        } else if mark > 100.0 {
            return Err(InvalidError("Mark must be below 100.0"));
        }

        self.mark = Some(mark);
        self.update_final_pct();
        Ok(())
    }

    /// Remove the mark for this [assignment](Assignment).
    ///
    /// # Example
    /// ```
    /// # use std::rc::Rc;
    /// # use tracker_core::{Assignment, ClassCode};
    /// let code = Rc::new(ClassCode::new("SOME101")?);
    /// let mut assign = Assignment::new("Test", 10.0, code)?;
    /// assign.set_mark(80.0)?;
    ///
    /// assign.remove_mark();
    /// assert_eq!(None, assign.mark());
    /// # Ok::<(), &'static str>(())
    /// ```
    pub fn remove_mark(&mut self) {
        self.mark = None;
        self.update_final_pct();
    }

    /// Get the value of the [assignment](Assignment) with regards to the final grade.
    ///
    /// # Example
    /// ```
    /// # use std::rc::Rc;
    /// # use tracker_core::{Assignment, ClassCode};
    /// let code = Rc::new(ClassCode::new("SOME101")?);
    /// let mut assign = Assignment::new("Test", 25.0, code)?;
    /// assert_eq!(25.0, assign.value());
    /// # Ok::<(), &'static str>(())
    /// ```
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Update the final percentage.
    fn update_final_pct(&mut self) {
        self.percent = self.mark().map(|m| (m / 100.0) * self.value())
    }

    /// Get the final grade contribution for this [assignment](Assignment).
    ///
    /// # Example
    /// ```
    /// # use std::rc::Rc;
    /// # use tracker_core::{Assignment, ClassCode};
    /// let code = Rc::new(ClassCode::new("SOME101")?);
    /// let mut assign = Assignment::new("Test", 10.0, code)?;
    ///
    /// /// Final percentage is `None` by default.
    /// assert_eq!(None, assign.final_pct());
    ///
    /// /// Setting the mark updates the final percentage.
    /// assign.set_mark(80.0)?;
    /// assert_eq!(8.0, assign.final_pct().unwrap());
    /// # Ok::<(), &'static str>(())
    /// ```
    pub fn final_pct(&self) -> Option<f64> {
        self.percent
    }

    /// Get the class code for this [assignment](Assignment).
    ///
    /// # Example
    /// ```
    /// # use std::rc::Rc;
    /// # use tracker_core::{Assignment, ClassCode};
    /// let code = Rc::new(ClassCode::new("SOME101")?);
    /// let mut assign = Assignment::new("Test", 10.0, Rc::clone(&code))?;
    /// assert_eq!(code, assign.class_code());
    /// # Ok::<(), &'static str>(())
    /// ```
    pub fn class_code(&self) -> Rc<ClassCode> {
        Rc::clone(&self.class_code)
    }

    /// Check if the assignment is valid.
    ///
    /// # Conditions
    /// - ```name``` length within range ```3..=20```
    /// - ```mark``` within range ```0..=100``` or ```None```
    /// - ```value``` within range ```0..=100```
    /// - ```percent``` within range ```0..=100``` or ```None```
    fn is_valid(&self) -> Result<()> {
        if !(3..=20).contains(&self.name().len()) {
            return Err(InvalidError(
                "Name must have at least 1 char and at most 20 chars",
            ));
        }

        if !(0.0..=100.0).contains(&self.value()) {
            return Err(InvalidError(
                "Value of an assignment should be in range 0..=100",
            ));
        }

        if let Some(m) = &self.mark() {
            if !(0.0..=100.0).contains(m) {
                return Err(InvalidError("Mark must be within range 0..=100 or None"));
            }
        }

        if let Some(p) = &self.final_pct() {
            if !(0.0..=100.0).contains(p) {
                return Err(InvalidError(
                    "Final percentage must be within range 0..=100 or None",
                ));
            }
        }

        Ok(())
    }
}

impl fmt::Display for Assignment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.mark() {
            Some(mark) => write!(
                f,
                "{} :: {} [Mark: {:.1} | Worth: {:.1} | Pct: {:.1}%]",
                self.class_code,
                self.name,
                mark,
                self.value,
                self.final_pct().unwrap()
            ),
            None => write!(
                f,
                "{} :: {} [No mark | Worth: {:.1}]",
                self.class_code, self.name, self.value
            ),
        }
    }
}

impl PartialOrd for Assignment {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.name.cmp(&other.name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_assignment_1() {
        let cc = ClassCode::new("SOME101").unwrap();
        let assign = Assignment::new("Test 1", 50.0, Rc::new(cc));
        assert!(assign.is_ok());
    }
    #[test]
    fn new_assignment_2() {
        let cc = ClassCode::new("SOME101").unwrap();
        let assign = Assignment::new("text is way longer than 20 chars", 101.0, Rc::new(cc));
        assert!(assign.is_err());
    }
    #[test]
    fn new_assignment_3() {
        let cc = ClassCode::new("SOME101").unwrap();
        let assign = Assignment::new("Test 1", 101.0, Rc::new(cc));
        assert!(assign.is_err());
    }
    #[test]
    fn new_assignment_4() {
        let cc = ClassCode::new("SOME101").unwrap();
        let assign = Assignment::new("Test 1", -50.0, Rc::new(cc));
        assert!(assign.is_err());
    }

    #[test]
    fn set_mark() {
        let cc = ClassCode::new("SOME101").unwrap();
        let mut assign = Assignment::new("Test 1", 50.0, Rc::new(cc)).unwrap();
        assert!(assign.set_mark(80.0).is_ok());
        assert!(assign.set_mark(-80.0).is_err());
        assert!(assign.set_mark(200.0).is_err());
    }
    #[test]
    fn final_pct() {
        let cc = ClassCode::new("SOME101").unwrap();
        let mut assign = Assignment::new("Test 1", 50.0, Rc::new(cc)).unwrap();
        assert!(assign.set_mark(100.0).is_ok());
        assert!(!assign.final_pct().is_none());
        assert_eq!(50.0, assign.final_pct().unwrap());

        assign.remove_mark();
        assert_eq!(None, assign.final_pct());
    }

    #[test]
    fn is_valid_1() {
        let cc = ClassCode::new("SOME101").unwrap();
        let assign = Assignment::new("Test 1", 50.0, Rc::new(cc)).unwrap();
        assert!(assign.is_valid().is_ok());
    }

    #[test]
    fn is_valid_2() {
        let cc = ClassCode::new("SOME101").unwrap();
        let mut assign = Assignment::new("Test 1", 50.0, Rc::new(cc)).unwrap();
        assign.set_mark(55.5).unwrap();
        assert!(assign.is_valid().is_ok());
    }
}
