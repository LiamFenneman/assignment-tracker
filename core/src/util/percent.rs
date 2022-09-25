use std::fmt;

/// A percentage. Integer value within the range 0 to 100.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Percent {
    value: u8,
}

impl Percent {
    /// Create a new percent.
    ///
    /// # Errors
    /// - If the `value` is greater than 100.
    pub fn new(value: u8) -> Result<Self, Error> {
        if value > 100 {
            Err(Error::OutOfRange(value))
        } else {
            Ok(Percent { value })
        }
    }

    /// Get the percent's value.
    #[must_use]
    pub fn value(&self) -> u8 {
        self.value
    }
}

impl fmt::Display for Percent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}%", self.value)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("percent value `{0}` is greater than 100")]
    OutOfRange(u8),
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(u8::MIN)]
    #[case(1)]
    #[case(50)]
    #[case(99)]
    #[case(100)]
    fn percent_new(#[case] pct: u8) {
        let percent = Percent::new(pct).unwrap();
        assert_eq!(percent.value(), pct);
    }

    #[rstest]
    #[case(101)]
    #[case(200)]
    #[case(u8::MAX)]
    fn percent_new_out_of_range(#[case] pct: u8) {
        let percent = Percent::new(pct);
        assert!(percent.is_err());
    }

    #[rstest]
    #[case(u8::MIN)]
    #[case(1)]
    #[case(50)]
    #[case(99)]
    #[case(100)]
    fn percent_display(#[case] pct: u8) {
        let percent = Percent::new(pct).unwrap();
        assert_eq!(percent.to_string(), format!("{}%", pct));
    }
}
