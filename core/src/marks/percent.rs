use std::fmt;

use crate::mark::OutOf;

/// A percentage. Integer value within the range 0 to 100.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

    #[must_use]
    pub const fn zero() -> Self {
        Percent { value: 0 }
    }

    /// Get the average of two percentages.
    #[must_use]
    pub fn average(self, other: Self) -> Self {
        let value = (self.value + other.value) / 2;
        Percent { value }
    }

    /// Get the average of many percentages.
    #[must_use]
    pub fn average_many(percents: impl IntoIterator<Item = Self>) -> Self {
        percents.into_iter().fold(Percent::zero(), Percent::average)
    }
}

impl From<OutOf> for Percent {
    fn from(out_of: OutOf) -> Self {
        let mark = out_of.mark();
        let out_of = out_of.out_of();
        let pct = (f32::from(mark) / f32::from(out_of)) * 100.0;
        assert!((0.0..=100.0).contains(&pct), "pct = {}", pct);
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        Percent::new(pct as u8).expect("pct already checked")
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
        let percent = Percent::new(pct).expect("invalid test cases");
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
    #[case(0, 100, 0)]
    #[case(1, 100, 1)]
    #[case(50, 200, 25)]
    #[case(100, 200, 50)]
    #[case(100, 300, 33)]
    fn percent_from_out_of(#[case] mark: u16, #[case] out_of: u16, #[case] pct: u8) {
        let out_of = OutOf::new(mark, out_of).expect("invalid test cases");
        let percent = Percent::from(out_of);
        assert_eq!(percent.value(), pct);
    }
}
