use std::error::Error;
use std::fmt;

/// Error occurs when parsing a string into Assignment.
#[derive(Debug)]
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
