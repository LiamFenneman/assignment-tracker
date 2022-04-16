mod invalid_error;
pub use invalid_error::InvalidError;

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{cmp, fmt, result};

use crate::ClassCode;

/// Representation of a single assignment.
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct Assignment {
    name: String,
    mark: Option<f64>,
    value: f64,
    percent: Option<f64>,
    class_code: ClassCode,
}

pub type Result<T> = result::Result<T, InvalidError>;

// use lazy static to create the regex once
lazy_static! {
    static ref RE: Regex = Regex::new(r"^[A-Z]{4}\d{3}$").unwrap();
}

impl Assignment {
    /// Create a new Assignment.
    ///
    /// # Conditions
    /// - ```name``` length within range ```3..=20```
    /// - ```value``` within range ```0..=100```
    ///
    /// # Examples
    /// ```
    /// use tracker_lib::{Assignment, ClassCode};
    /// let valid = Assignment::new("Test", 10.0, ClassCode::new("SOME101").unwrap());
    /// assert!(valid.is_ok());
    /// ```
    pub fn new(name: &str, value: f64, class_code: ClassCode) -> Result<Self> {
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

    /// Get the name of the assignment.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Get the mark for the assignment.
    pub fn mark(&self) -> Option<f64> {
        self.mark
    }

    /// Set the mark for the assignment.
    ///
    /// # Conditions
    /// If the mark is not ```None``` then both ```m >= 0.0``` and ```m <= 100.0``` must hold for the mark to be set.
    ///
    /// # Examples
    /// ```
    /// use tracker_lib::{Assignment, ClassCode};
    /// let mut assign = Assignment::new("Test", 10.0, ClassCode::new("SOME101").unwrap()).unwrap();
    /// assert!(assign.set_mark(80.0).is_ok());
    /// assert!(assign.set_mark(-80.0).is_err());
    /// assert!(assign.set_mark(200.0).is_err());
    /// ```
    pub fn set_mark(&mut self, mark: f64) -> Result<()> {
        if mark < 0.0 {
            return Err(InvalidError::with_msg("Mark must be positive"));
        } else if mark > 100.0 {
            return Err(InvalidError::with_msg("Mark must be below 100.0"));
        }

        self.mark = Some(mark);
        self.update_final_pct();
        Ok(())
    }

    /// Remove the mark for this assignment.
    pub fn remove_mark(&mut self) {
        self.mark = None;
        self.update_final_pct();
    }

    /// Get the value of the assignment with regards to the final grade.
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Update the final percentage.
    fn update_final_pct(&mut self) {
        self.percent = match self.mark() {
            Some(m) => Some((m / 100.0) * self.value()),
            None => None,
        }
    }

    /// Get the final grade contribution for this assignment.
    pub fn final_pct(&self) -> Option<f64> {
        self.percent
    }

    /// Get the class code for this assignment.
    pub fn class_code(&self) -> &ClassCode {
        &self.class_code
    }

    /// Serialize the Assignment into JSON format.
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).expect("Problem with serialization")
    }

    /// Check if the assignment is valid.
    ///
    /// # Conditions
    /// - ```name``` length within range ```3..=20```
    /// - ```mark``` within range ```0..=100``` or ```None```
    /// - ```value``` within range ```0..=100```
    /// - ```percent``` within range ```0..=100``` or ```None```
    pub fn is_valid(&self) -> Result<()> {
        if !(3..=20).contains(&self.name().len()) {
            return Err(InvalidError::with_msg(
                "Name must have at least 1 char and at most 20 chars",
            ));
        }

        if !(0.0..=100.0).contains(&self.value()) {
            return Err(InvalidError::with_msg(
                "Value of an assignment should be in range 0..=100",
            ));
        }

        if let Some(m) = &self.mark() {
            if !(0.0..=100.0).contains(m) {
                return Err(InvalidError::with_msg(
                    "Mark must be within range 0..=100 or None",
                ));
            }
        }

        if let Some(p) = &self.final_pct() {
            if !(0.0..=100.0).contains(p) {
                return Err(InvalidError::with_msg(
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
        let assign = Assignment::new("Test 1", 50.0, ClassCode::new("SOME101").unwrap());
        assert!(assign.is_ok());
    }
    #[test]
    fn new_assignment_2() {
        let assign = Assignment::new(
            "text is way longer than 20 chars",
            101.0,
            ClassCode::new("SOME101").unwrap(),
        );
        assert!(assign.is_err());
    }
    #[test]
    fn new_assignment_3() {
        let assign = Assignment::new("Test 1", 101.0, ClassCode::new("SOME101").unwrap());
        assert!(assign.is_err());
    }
    #[test]
    fn new_assignment_4() {
        let assign = Assignment::new("Test 1", -50.0, ClassCode::new("SOME101").unwrap());
        assert!(assign.is_err());
    }

    #[test]
    fn set_mark() {
        let mut assign =
            Assignment::new("Test 1", 50.0, ClassCode::new("SOME101").unwrap()).unwrap();
        assert!(assign.set_mark(80.0).is_ok());
        assert!(assign.set_mark(-80.0).is_err());
        assert!(assign.set_mark(200.0).is_err());
    }
    #[test]
    fn final_pct() {
        let mut assign =
            Assignment::new("Test 1", 50.0, ClassCode::new("SOME101").unwrap()).unwrap();
        assert!(assign.set_mark(100.0).is_ok());
        assert!(!assign.final_pct().is_none());
        assert_eq!(50.0, assign.final_pct().unwrap());

        assign.remove_mark();
        assert_eq!(None, assign.final_pct());
    }

    #[test]
    fn is_valid_1() {
        let assign = Assignment::new("Test 1", 50.0, ClassCode::new("SOME101").unwrap()).unwrap();
        assert!(assign.is_valid().is_ok());
    }

    #[test]
    fn is_valid_2() {
        let mut assign =
            Assignment::new("Test 1", 50.0, ClassCode::new("SOME101").unwrap()).unwrap();
        assign.set_mark(55.5).unwrap();
        assert!(assign.is_valid().is_ok());
    }
}
