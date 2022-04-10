use std::{cmp, env, error::Error, fmt, str::FromStr};

/// Wrapper for arguments passed to the program.
pub struct Args {
    filename: String,
}

impl Args {
    /// Convert from std::env::Args to Args.
    /// Ensuring that a filename is passed as the first argument.
    pub fn new(mut args: env::Args) -> Result<Self, &'static str> {
        args.next(); // ignore 1st arg (program name)

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't provide a filename"),
        };

        Ok(Self { filename })
    }

    /// Get access to the filename as a immutable reference.
    pub fn filename(&self) -> &str {
        &self.filename
    }
}

#[derive(PartialEq, Debug)]
pub struct Assignment {
    name: String,
    mark: Option<f64>,
    value: f64,
    class_code: String,
}

impl Assignment {
    /// Create a new Assignment.
    ///
    /// # Conditions
    /// - ```name``` length within range ```3..=20```
    /// - ```value``` within range ```0..=100```
    /// - ```class``` format: ```XXXX###``` (e.g. SOME101)
    ///
    /// # Examples
    /// ```
    /// let valid = tracker::Assignment::new("Test", 10.0, "SOME101");
    /// assert!(valid.is_ok());
    /// ```
    pub fn new(name: &str, value: f64, class_code: &str) -> Result<Self, &'static str> {
        if !(3..=20).contains(&name.len()) {
            return Err("Name must have at least 1 char and at most 20 chars");
        }

        let re = regex::Regex::new(r"^[A-Z]{4}\d{3}$").unwrap_or_else(|err| {
            panic!("Regex error: {}", err);
        });

        if !re.is_match(&class_code) {
            return Err("Class code doesn't follow format: XXXX### (e.g. SOME101)");
        }

        if !(0.0..=100.0).contains(&value) {
            return Err("Value of an assignment should be in range 0..=100");
        }

        Ok(Self {
            name: name.to_string(),
            mark: None,
            value,
            class_code: class_code.to_string(),
        })
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
    /// # Example
    /// ```
    /// let mut assign = tracker::Assignment::new("Test 1", 50.0, "SOME101").unwrap();
    /// assert!(assign.set_mark(80.0).is_ok());
    /// assert!(assign.set_mark(-80.0).is_err());
    /// assert!(assign.set_mark(200.0).is_err());
    /// ```
    pub fn set_mark(&mut self, mark: f64) -> Result<(), &'static str> {
        if mark < 0.0 {
            return Err("Mark must be positive");
        } else if mark > 100.0 {
            return Err("Mark must be below 100.0");
        }

        self.mark = Some(mark);
        Ok(())
    }

    /// Remove the mark for this assignment.
    pub fn remove_mark(&mut self) {
        self.mark = None;
    }

    /// Get the value of the assignment with regards to the final grade.
    pub fn value(&self) -> f64 {
        assert!(self.value >= 0.0 && self.value <= 100.0);
        self.value
    }

    /// Get the final grade contribution for this assignment.
    pub fn final_pct(&self) -> Option<f64> {
        match self.mark() {
            Some(m) => Some((m / 100.0) * self.value()),
            None => None,
        }
    }

    /// Get the class code for this assignment.
    pub fn class_code(&self) -> &str {
        &self.class_code
    }

    /// Get the Assignment in CSV format.
    ///
    /// # Examples
    /// ```
    /// let mut a = tracker::Assignment::new("Test 1", 25.0, "TEST123").unwrap();
    /// assert_eq!("TEST123,Test 1,None,25.0", a.as_csv());
    /// a.set_mark(99.9);
    /// assert_eq!("TEST123,Test 1,99.9,25.0", a.as_csv());
    /// ```
    pub fn as_csv(&self) -> String {
        match self.mark() {
            Some(mark) => format!(
                "{},{},{:.1},{:.1}",
                self.class_code(),
                self.name(),
                mark,
                self.value()
            ),
            None => format!(
                "{},{},None,{:.1}",
                self.class_code(),
                self.name(),
                self.value()
            ),
        }
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

// Allow for parsing to Assignment from CSV
impl FromStr for Assignment {
    type Err = AssignmentParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.trim().split(',').collect();

        // parse the value into f64
        let value: f64 = split[3].parse().unwrap();

        // create an assignment, if err then return a parse error
        let mut assignment = match Assignment::new(split[1], value, split[0]) {
            Ok(m) => m,
            Err(e) => return Err(AssignmentParseError::new(&format! {"{:?}", e})),
        };

        // if the split for mark is not "None" then try parse f64
        if split[2] != "None" {
            let mark: f64 = match split[2].parse() {
                Ok(m) => m,
                Err(e) => return Err(AssignmentParseError::new(&format!("{:?}", e))),
            };
            // try set the mark
            if let Err(e) = assignment.set_mark(mark) {
                return Err(AssignmentParseError::new(&format!("{:?}", e)));
            }
        }

        Ok(assignment)
    }
}

/// Error occurs when parsing a string into Assignment.
pub struct AssignmentParseError {
    msg: String,
}

impl AssignmentParseError {
    pub fn new(msg: &str) -> Self {
        Self {
            msg: msg.to_string(),
        }
    }
}

impl Error for AssignmentParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl fmt::Display for AssignmentParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error parsing assignment: {}", self.msg)
    }
}

impl fmt::Debug for AssignmentParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AssignmentParseError {{ msg: {} }}", self.msg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_assignment_1() {
        let assign = Assignment::new("Test 1", 50.0, "SOME101");
        assert!(assign.is_ok());
    }
    #[test]
    fn new_assignment_2() {
        let assign = Assignment::new("text is way longer than 20 chars", 101.0, "some text");
        assert!(assign.is_err());
    }
    #[test]
    fn new_assignment_3() {
        let assign = Assignment::new("Test 1", 101.0, "some text");
        assert!(assign.is_err());
    }
    #[test]
    fn new_assignment_4() {
        let assign = Assignment::new("Test 1", -50.0, "some text");
        assert!(assign.is_err());
    }
    #[test]
    fn new_assignment_5() {
        let assign = Assignment::new("Test 1", 50.0, "some text");
        assert!(assign.is_err());
    }

    #[test]
    fn set_mark() {
        let mut assign = Assignment::new("Test 1", 50.0, "SOME101").unwrap();
        assert!(assign.set_mark(80.0).is_ok());
        assert!(assign.set_mark(-80.0).is_err());
        assert!(assign.set_mark(200.0).is_err());
    }
    #[test]
    fn final_pct() {
        let mut assign = Assignment::new("Test 1", 50.0, "SOME101").unwrap();
        assert!(assign.set_mark(100.0).is_ok());
        assert!(!assign.final_pct().is_none());
        assert_eq!(50.0, assign.final_pct().unwrap());

        assign.remove_mark();
        assert_eq!(None, assign.final_pct());
    }

    #[test]
    fn parse_valid_1() {
        let a: Result<Assignment, _> = "TEST101,Test 1,75.0,20.0".parse();
        assert!(a.is_ok());
        let a = a.unwrap();
        assert_eq!(
            Assignment {
                name: String::from("Test 1"),
                mark: Some(75.0),
                value: 20.0,
                class_code: String::from("TEST101")
            },
            a
        );
    }
    #[test]
    fn parse_valid_2() {
        let a: Result<Assignment, _> = "TEST101,Test 1,None,20.0".parse();
        assert!(a.is_ok());
        let a = a.unwrap();
        assert_eq!(
            Assignment {
                name: String::from("Test 1"),
                mark: None,
                value: 20.0,
                class_code: String::from("TEST101")
            },
            a
        );
    }
    #[test]
    fn parse_invalid_1() {
        let a: Result<Assignment, _> = ",Test 1,None,20.0".parse();
        assert!(a.is_err());
    }
    #[test]
    fn parse_invalid_2() {
        let a: Result<Assignment, _> = "NOT 101,Test 1,None,20.0".parse();
        assert!(a.is_err());
    }
    #[test]
    fn parse_invalid_3() {
        let a: Result<Assignment, _> = "TEST___,Test 1,None,20.0".parse();
        assert!(a.is_err());
    }
    #[test]
    fn parse_invalid_4() {
        let a: Result<Assignment, _> = "TEST101,,None,20.0".parse();
        assert!(a.is_err());
    }
    #[test]
    fn parse_invalid_5() {
        let a: Result<Assignment, _> =
            "TEST101,really long assignment name which is invalid,None,20.0".parse();
        assert!(a.is_err());
    }
    #[test]
    fn parse_invalid_6() {
        let a: Result<Assignment, _> = "TEST101,Aa,None,20.0".parse();
        assert!(a.is_err());
    }
    #[test]
    fn parse_invalid_7() {
        let a: Result<Assignment, _> = "TEST101,Test 1,-10.0,20.0".parse();
        assert!(a.is_err());
    }
    #[test]
    fn parse_invalid_8() {
        let a: Result<Assignment, _> = "TEST101,Test 1,110.0,20.0".parse();
        assert!(a.is_err());
    }
    #[test]
    fn parse_invalid_9() {
        let a: Result<Assignment, _> = "TEST101,Test 1,80.0,-20.0".parse();
        assert!(a.is_err());
    }
    #[test]
    fn parse_invalid_10() {
        let a: Result<Assignment, _> = "TEST101,Test 1,80.0,120.0".parse();
        assert!(a.is_err());
    }
}
