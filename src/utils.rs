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
    pub fn set_mark(&mut self, mark: Option<f64>) -> Result<(), &'static str> {
        match mark {
            Some(m) => {
                if m < 0.0 {
                    return Err("Mark must be positive");
                } else if m > 100.0 {
                    return Err("Mark must be below 100.0");
                }

                self.mark = mark;
                Ok(())
            }
            None => Ok(()),
        }
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
