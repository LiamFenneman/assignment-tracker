use std::rc::Rc;

use crate::{class_code::ClassCodes, Assignment, ClassCode};

/// Track assignments.
#[derive(Debug, PartialEq, PartialOrd)]
pub struct Tracker {
    list: Vec<Assignment>,
    codes: ClassCodes,
}

type ValidResult = Result<(), &'static str>;

impl Tracker {
    /// Create a new default tracker.
    ///
    /// # Example
    /// ```
    /// # use tracker_core::Tracker;
    /// let tracker = Tracker::new();
    /// assert_eq!(0, tracker.get_all().len());
    /// ```
    pub fn new() -> Self {
        let list = Vec::new();
        let codes = ClassCodes::new();
        Self { list, codes }
    }

    /// Add an [assignment](Assignment) to be tracked.
    ///
    /// # Example
    /// ```
    /// # use tracker_core::{Assignment, Tracker};
    /// let mut tracker = Tracker::new();
    /// let assign = Assignment::new("Test", 10.0, tracker.get_code("TEST123")?)?;
    /// tracker.track(assign)?;
    /// assert_eq!(1, tracker.get_all().len());
    /// # Ok::<(), &'static str>(())
    /// ```
    pub fn track(&mut self, assign: Assignment) -> ValidResult {
        for a in self.list.iter() {
            // check for duplicate assignments
            if assign == *a {
                return Err("Cannot add duplicate assignments");
            }
        }

        // ensure the total value of all assignments in the class are below 100.0
        let total_value: f64 = self
            .get_all_from_class(assign.class_code())
            .iter()
            .map(|a| a.value())
            .sum();
        if total_value + assign.value() > 100.0 {
            return Err("Total value of assignments in the class will be more than 100.0");
        }

        self.list.push(assign);
        Ok(())
    }

    /// Add many [assignments](Assignment) to be tracked.
    ///
    /// # Example
    /// ```
    /// # use std::rc::Rc;
    /// # use tracker_core::{Assignment, Tracker};
    /// let mut tracker = Tracker::new();
    /// let code = tracker.get_code("TEST123")?;
    /// let assigns = vec![
    ///     Assignment::new("Test 1", 10.0, Rc::clone(&code))?,
    ///     Assignment::new("Test 2", 10.0, Rc::clone(&code))?,
    ///     Assignment::new("Test 3", 10.0, Rc::clone(&code))?,
    /// ];
    /// tracker.track_many(assigns)?;
    /// assert_eq!(3, tracker.get_all().len());
    /// # Ok::<(), &'static str>(())
    /// ```
    pub fn track_many(&mut self, ass: Vec<Assignment>) -> ValidResult {
        for a in ass {
            self.track(a)?;
        }

        Ok(())
    }

    /// Untrack an [assignment](Assignment) with the given name and [class code](ClassCode).
    ///
    /// # Example
    /// ***Note:*** *this example contains the example from [`Tracker::track_many()`]*
    /// ```
    /// # use std::rc::Rc;
    /// # use tracker_core::{Assignment, Tracker};
    /// # let mut tracker = Tracker::new();
    /// let code = tracker.get_code("TEST123")?;
    /// # let assigns = vec![
    /// #     Assignment::new("Test 1", 10.0, Rc::clone(&code))?,
    /// #     Assignment::new("Test 2", 10.0, Rc::clone(&code))?,
    /// #     Assignment::new("Test 3", 10.0, Rc::clone(&code))?,
    /// # ];
    /// # tracker.track_many(assigns)?;
    /// # assert_eq!(3, tracker.get_all().len());
    ///
    /// /// Assignment exists
    /// assert!(tracker.untrack("Test 1", Rc::clone(&code)).is_ok());
    /// assert_eq!(2, tracker.get_all().len());
    ///
    /// /// Assignment doesn't exist
    /// assert!(tracker.untrack("Something Else", Rc::clone(&code)).is_err());
    /// # Ok::<(), &'static str>(())
    /// ```
    pub fn untrack(&mut self, name: &str, class: Rc<ClassCode>) -> ValidResult {
        // filter out assignments
        let filtered: Vec<&Assignment> = self
            .list
            .iter()
            .filter(|a| a.name() == name && *a.class_code() == *class)
            .collect();

        let len = filtered.len();

        if len == 0 {
            return Err("Could not find any assignment");
        }

        if len > 1 {
            panic!("There are assignments with the same name and class");
        }

        if let Some(i) = self.list.iter().position(|r| r == filtered[0]) {
            self.list.remove(i);
            return Ok(());
        }

        panic!("Could not find the index of the assignment");
    }

    /// Get a reference to all the [assignments](Assignment) which are tracked.
    ///
    /// # Example
    /// ***Note:*** *this example contains the example from [`Tracker::track_many()`]*
    /// ```
    /// # use std::rc::Rc;
    /// # use tracker_core::{Assignment, Tracker};
    /// # let mut tracker = Tracker::new();
    /// # let code = tracker.get_code("TEST123")?;
    /// # let assigns = vec![
    /// #     Assignment::new("Test 1", 10.0, Rc::clone(&code))?,
    /// #     Assignment::new("Test 2", 10.0, Rc::clone(&code))?,
    /// #     Assignment::new("Test 3", 10.0, Rc::clone(&code))?,
    /// # ];
    /// # tracker.track_many(assigns.clone())?;
    /// let all = tracker.get_all();
    /// assert_eq!(&assigns, all);
    /// # Ok::<(), &'static str>(())
    /// ```
    pub fn get_all(&self) -> &Vec<Assignment> {
        &self.list
    }

    /// Get a reference to all the [assignments](Assignment) which belong to a given [class](ClassCode).
    ///
    /// # Example
    /// ```
    /// # use std::rc::Rc;
    /// # use tracker_core::{Assignment, Tracker};
    /// let mut tracker = Tracker::new();
    /// let code1 = tracker.get_code("TEST123")?;
    /// let code2 = tracker.get_code("OTHR456")?;
    /// let assigns = vec![
    ///     /// Assignments for class TEST123
    ///     Assignment::new("Test 1", 10.0, Rc::clone(&code1))?,
    ///     Assignment::new("Test 2", 10.0, Rc::clone(&code1))?,
    ///     Assignment::new("Test 3", 10.0, Rc::clone(&code1))?,
    ///     /// Assignments for class OTHR456
    ///     Assignment::new("Other Test 1", 10.0, Rc::clone(&code2))?,
    ///     Assignment::new("Other Test 2", 10.0, Rc::clone(&code2))?,
    /// ];
    /// tracker.track_many(assigns.clone())?;
    ///
    /// let all_code1 = tracker.get_all_from_class(Rc::clone(&code1));
    /// let all_code2 = tracker.get_all_from_class(Rc::clone(&code2));
    /// assert_eq!(3, all_code1.len());
    /// assert_eq!(2, all_code2.len());
    ///
    /// # Ok::<(), &'static str>(())
    /// ```
    pub fn get_all_from_class(&self, class: Rc<ClassCode>) -> Vec<&Assignment> {
        self.list
            .iter()
            .filter(|a| *a.class_code() == *class)
            .collect()
    }

    /// Get [class code](ClassCode) from string literal.
    ///
    /// See [`ClassCode::new()`] for string literal format.
    ///
    /// # Example
    /// ```
    /// # use tracker_core::Tracker;
    /// let mut tracker = Tracker::new();
    /// assert!(tracker.get_code("TEST123").is_ok());
    /// assert!(tracker.get_code("TEST456").is_ok());
    /// assert!(tracker.get_code("TEST789").is_ok());
    /// ```
    pub fn get_code(&mut self, str: &str) -> Result<Rc<ClassCode>, &'static str> {
        let cc = self.codes.get(str)?;
        Ok(Rc::clone(&cc))
    }
}

impl Default for Tracker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let tracker = Tracker::new();
        let empty: Vec<Assignment> = Vec::new();
        assert_eq!(empty, tracker.list);
    }

    #[test]
    fn track_valid() {
        let mut tracker = Tracker::new();
        let a =
            Assignment::new("Assignment 1", 25.0, tracker.get_code("SOME101").unwrap()).unwrap();
        let b = a.clone();
        let track = tracker.track(a);
        assert!(track.is_ok());
        assert_eq!(vec![b], tracker.list);
        assert_eq!(1, tracker.list.len());
    }

    #[test]
    fn track_invalid_1() {
        let mut tracker = Tracker::new();
        let a =
            Assignment::new("Assignment 1", 25.0, tracker.get_code("SOME101").unwrap()).unwrap();
        let b = a.clone();
        let r = tracker.track_many(vec![a, b]);
        assert!(r.is_err())
    }

    #[test]
    fn track_invalid_2() {
        let mut tracker = Tracker::new();
        let a =
            Assignment::new("Assignment 1", 75.0, tracker.get_code("SOME101").unwrap()).unwrap();
        let b =
            Assignment::new("Assignment 2", 50.0, tracker.get_code("SOME101").unwrap()).unwrap();
        let r = tracker.track_many(vec![a, b]);
        assert!(r.is_err())
    }

    #[test]
    fn untrack_valid() {
        let mut tracker = gen_tracker(3);
        assert_eq!(3, tracker.list.len());
        let code = tracker.get_code("TEST123").unwrap();
        let r = tracker.untrack("Assignment 2", code);
        assert!(r.is_ok());
        assert_eq!(2, tracker.list.len());
    }

    #[test]
    fn untrack_invalid_1() {
        let mut tracker = gen_tracker(3);
        assert_eq!(3, tracker.list.len());
        let code = tracker.get_code("TEST123").unwrap();
        let r = tracker.untrack("Assignment", code);
        assert!(r.is_err());
        assert_eq!(3, tracker.list.len());
    }

    #[test]
    fn untrack_invalid_2() {
        let mut tracker = gen_tracker(3);
        assert_eq!(3, tracker.list.len());
        let code = tracker.get_code("TEST123").unwrap();
        let r = tracker.untrack("Assignment 3", code);
        assert!(r.is_err());
        assert_eq!(3, tracker.list.len());
    }

    #[test]
    fn from_class() {
        let mut tracker = gen_tracker(3);
        let code = tracker.get_code("OTHR456").unwrap();
        tracker
            .track(Assignment::new("Test 1", 50.0, Rc::clone(&code)).unwrap())
            .unwrap();
        tracker
            .track(Assignment::new("Test 2", 50.0, Rc::clone(&code)).unwrap())
            .unwrap();
        assert_eq!(5, tracker.get_all().len());
        let code = tracker.get_code("TEST123").unwrap();
        assert_eq!(3, tracker.get_all_from_class(code).len());
        let code = tracker.get_code("OTHR456").unwrap();
        assert_eq!(2, tracker.get_all_from_class(code).len());
    }

    fn gen_tracker(size: usize) -> Tracker {
        let mut tracker = Tracker::new();
        let cc = tracker.get_code("TEST123").unwrap();
        for i in 0..size {
            let code = Rc::clone(&cc);
            tracker
                .track(Assignment::new(&format!("Assignment {}", i), 10.0, code).unwrap())
                .unwrap();
        }
        tracker
    }
}
