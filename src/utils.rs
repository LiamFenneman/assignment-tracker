use std::{cmp, env, fmt};

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

/// The class which an assignment can belong to.
#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct Class {
    pub code: String,
}

impl Class {
    /// Create a new class.
    ///
    /// Code format: 4 capital letters + 3 digits
    ///
    /// # Examples
    /// Valid code format:
    /// ```
    /// let class = tracker::Class::new("SOME101");
    /// assert!(class.is_ok());
    /// ```
    /// Invalid code format:
    /// ```
    /// let class = tracker::Class::new("some random text");
    /// assert!(class.is_err());
    /// ```
    pub fn new(code: &str) -> Result<Self, &'static str> {
        let re = match regex::Regex::new(r"^[A-Z]{4}\d{3}$") {
            Ok(re) => re,
            Err(_) => return Err("Regex error"),
        };

        if !re.is_match(&code) {
            return Err("Code doesn't follow format: XXXX### (e.g. EXPL101)");
        }

        Ok(Self {
            code: code.to_string(),
        })
    }
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code)
    }
}

#[derive(PartialEq)]
pub struct Assignment {
    name: String,
    mark: Option<f64>,
    value: f64,
    class: Class,
}

impl Assignment {
    /// Create a new Assignment.
    pub fn new(name: &str, value: f64, class: Class) -> Self {
        Self {
            name: name.to_string(),
            mark: None,
            value,
            class,
        }
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
    /// let class = tracker::Class::new("SOME101").unwrap();
    /// let mut assign = tracker::Assignment::new("Test 1", 50.0, class);
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
}

impl fmt::Display for Assignment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.mark() {
            Some(mark) => write!(
                f,
                "{}::{} [Mark: {} | Worth: {} | Pct: {:?}%]",
                self.class,
                self.name,
                mark,
                self.value,
                self.final_pct().unwrap()
            ),
            None => write!(
                f,
                "{}::{} [No mark | Worth: {}]",
                self.class, self.name, self.value
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
    fn new_class() {
        let class = Class::new("SOME101");
        assert!(class.is_ok());

        let class = Class::new("some random text");
        assert!(class.is_err())
    }

    #[test]
    fn set_mark() {
        let mut assign = Assignment::new("Test 1", 50.0, Class::new("SOME101").unwrap());
        assert!(assign.set_mark(80.0).is_ok());
        assert!(assign.set_mark(-80.0).is_err());
        assert!(assign.set_mark(200.0).is_err());
    }
    #[test]
    fn final_pct() {
        let mut assign = Assignment::new("Test 1", 50.0, Class::new("SOME101").unwrap());
        assert!(assign.set_mark(100.0).is_ok());
        assert!(!assign.final_pct().is_none());
        assert_eq!(50.0, assign.final_pct().unwrap());

        assign.remove_mark();
        assert_eq!(None, assign.final_pct());
    }
}
