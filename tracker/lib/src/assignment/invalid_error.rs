use std::error::Error;
use std::fmt;

/// Error for when the Assignment is invalid.
#[derive(Debug)]
pub struct InvalidError(pub &'static str);

impl Error for InvalidError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl fmt::Display for InvalidError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Assignment is invalid: {}", self.0)
    }
}

impl From<InvalidError> for &str {
    fn from(_: InvalidError) -> Self {
        "Invalid error"
    }
}
