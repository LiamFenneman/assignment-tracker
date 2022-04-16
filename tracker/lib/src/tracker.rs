use crate::{Assignment, ClassCode};

/// Track assignments.
#[derive(Debug, PartialEq, PartialOrd)]
pub struct Tracker {
    list: Vec<Assignment>,
}

type ValidResult = Result<(), &'static str>;

impl Tracker {
    /// Create a new default tracker.
    pub fn new() -> Self {
        let list = Vec::new();
        Self { list }
    }

    /// Add an assignment to be tracked.
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

    /// Add many assignments to be tracked.
    pub fn track_many(&mut self, ass: Vec<Assignment>) -> ValidResult {
        for a in ass {
            self.track(a)?;
        }

        Ok(())
    }

    /// Untrack an assignment with the given name and class code.
    pub fn untrack(&mut self, name: &str, class: &ClassCode) -> ValidResult {
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

    /// Get a reference to all the assignments which are tracked.
    pub fn get_all(&self) -> &Vec<Assignment> {
        &self.list
    }

    /// Get a reference to all the assignments which belong to a given class.
    pub fn get_all_from_class(&self, class: &ClassCode) -> Vec<&Assignment> {
        self.list
            .iter()
            .filter(|a| *a.class_code() == *class)
            .collect()
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
        let a = Assignment::new("Assignment 1", 25.0, ClassCode::new("SOME101").unwrap()).unwrap();
        let b = a.clone();
        let track = tracker.track(a);
        assert!(track.is_ok());
        assert_eq!(vec![b], tracker.list);
        assert_eq!(1, tracker.list.len());
    }

    #[test]
    fn track_invalid_1() {
        let mut tracker = Tracker::new();
        let a = Assignment::new("Assignment 1", 25.0, ClassCode::new("SOME101").unwrap()).unwrap();
        let b = a.clone();
        let r = tracker.track_many(vec![a, b]);
        assert!(r.is_err())
    }

    #[test]
    fn track_invalid_2() {
        let mut tracker = Tracker::new();
        let a = Assignment::new("Assignment 1", 75.0, ClassCode::new("SOME101").unwrap()).unwrap();
        let b = Assignment::new("Assignment 2", 50.0, ClassCode::new("SOME101").unwrap()).unwrap();
        let r = tracker.track_many(vec![a, b]);
        assert!(r.is_err())
    }

    #[test]
    fn untrack_valid() {
        let mut tracker = gen_tracker(3);
        assert_eq!(3, tracker.list.len());
        let r = tracker.untrack("Assignment 2", &ClassCode::new("TEST123").unwrap());
        assert!(r.is_ok());
        assert_eq!(2, tracker.list.len());
    }

    #[test]
    fn untrack_invalid_1() {
        let mut tracker = gen_tracker(3);
        assert_eq!(3, tracker.list.len());
        let r = tracker.untrack("Assignment", &ClassCode::new("SOME101").unwrap());
        assert!(r.is_err());
        assert_eq!(3, tracker.list.len());
    }

    #[test]
    fn untrack_invalid_2() {
        let mut tracker = gen_tracker(3);
        assert_eq!(3, tracker.list.len());
        let r = tracker.untrack("Assignment 3", &ClassCode::new("OTHR222").unwrap());
        assert!(r.is_err());
        assert_eq!(3, tracker.list.len());
    }

    #[test]
    fn from_class() {
        let mut tracker = gen_tracker(3);
        tracker
            .track(Assignment::new("Test 1", 50.0, ClassCode::new("OTHR456").unwrap()).unwrap())
            .unwrap();
        tracker
            .track(Assignment::new("Test 2", 50.0, ClassCode::new("OTHR456").unwrap()).unwrap())
            .unwrap();
        assert_eq!(5, tracker.get_all().len());
        assert_eq!(
            3,
            tracker
                .get_all_from_class(&ClassCode::new("TEST123").unwrap())
                .len()
        );
        assert_eq!(
            2,
            tracker
                .get_all_from_class(&ClassCode::new("OTHR456").unwrap())
                .len()
        );
    }

    fn gen_tracker(size: usize) -> Tracker {
        let mut tracker = Tracker::new();
        for i in 0..size {
            let cc = ClassCode::new("TEST123").unwrap();
            tracker
                .track(Assignment::new(&format!("Assignment {}", i), 10.0, cc).unwrap())
                .unwrap();
        }
        tracker
    }
}
