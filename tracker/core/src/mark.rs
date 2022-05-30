use std::fmt::Display;
use thiserror::Error;

/// Type of mark with value.
/// Different [assignments](crate::prelude::Assignmentlike) can use different marking systems.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum Mark {
    /// Percentage value.
    ///
    /// Ensure validity with [`Mark::percent()`]
    ///
    /// # Examples
    /// ```
    /// # use tracker_core::prelude::Mark;
    /// let a = Mark::Percent(85.0);  // 85%
    /// let b = Mark::Percent(72.25); // 72.25%
    ///
    /// // or with validity checking
    /// let c = Mark::percent(85.0).unwrap();  // 85%
    /// let d = Mark::percent(72.25).unwrap(); // 72.25%
    ///
    /// assert_eq!(a, c);
    /// assert_eq!(b, d);
    ///
    /// // validity checking rejects the following
    /// assert!(Mark::percent(-50.0).is_err());
    /// assert!(Mark::percent(105.0).is_err());
    /// ```
    Percent(f64),
    /// Letter grade.
    ///
    /// Ensure validity with [`Mark::letter()`]
    ///
    /// # Examples
    /// ```
    /// # use tracker_core::prelude::Mark;
    /// let a = Mark::Letter('A');
    /// let b = Mark::Letter('B');
    ///
    /// // or with validity checking
    /// let c = Mark::letter('A').unwrap();
    /// let d = Mark::letter('B').unwrap();
    ///
    /// assert_eq!(a, c);
    /// assert_eq!(b, d);
    ///
    /// // validity checking rejects the following
    /// assert!(Mark::letter('a').is_err());
    /// assert!(Mark::letter('0').is_err());
    /// assert!(Mark::letter('%').is_err());
    /// assert!(Mark::letter('$').is_err());
    /// ```
    Letter(char),
    /// X out of Y value.
    ///
    /// Ensure validity with [`Mark::out_of()`]
    ///
    /// # Examples
    /// ```
    /// # use tracker_core::prelude::Mark;
    /// let a = Mark::OutOf(15, 20); // 15 marks out of 20
    /// let b = Mark::OutOf(13, 15); // 13 marks out of 15
    ///
    /// // or with validity checking
    /// let c = Mark::out_of(15, 20).unwrap(); // 15 marks out of 20
    /// let d = Mark::out_of(13, 15).unwrap(); // 13 marks out of 15
    ///
    /// assert_eq!(a, c);
    /// assert_eq!(b, d);
    ///
    /// // validity checking rejects the following
    /// assert!(Mark::out_of(10, 9).is_err());
    /// ```
    OutOf(u32, u32),
}

type MarkResult = Result<Mark, InvalidMarkError>;

impl Mark {
    /// Check if the mark is valid.
    ///
    /// # Errors
    /// - [`Mark::Percent`]: value is within range `0.0..=100.0`
    /// - [`Mark::Letter`]: [`char`] must be within range `A..=Z`
    /// - [`Mark::OutOf`]: *X* is less than or equal to *Y*
    pub fn check_valid(&self) -> Result<(), InvalidMarkError> {
        match self {
            Self::Percent(pct) if !(0.0..=100.0).contains(pct) => {
                if (0.0..=0.1).contains(pct) {
                    warn!("Percent range is 0.0 to 100.0 -> Provided value ({pct}) might not be correct.");
                }
                Err(InvalidMarkError::PercentOutOfRange(*pct))
            }
            Self::Letter(c) if !('A'..='Z').contains(c) => {
                Err(InvalidMarkError::LetterOutOfRange(*c))
            }
            Self::OutOf(a, b) if a > b => Err(InvalidMarkError::OutOfTupleEquality(*a, *b)),
            _ => Ok(()),
        }
    }

    /// Create a **valid** [`Mark::Percent`] with the provided [`f64`].
    ///
    /// # Errors
    /// - `pct` is **not** within range `0.0..=100.0`
    ///
    /// # Warnings
    /// - `pct` is within range `0.0..0.1`:
    ///     - A value this low is unlikely to occur due to it being less than 0.1%
    ///     - The value should be rounded to `0.0` or `0.1`
    pub fn percent(pct: f64) -> MarkResult {
        if (0.0..=0.1).contains(&pct) {
            warn!("Percent range is 0.0 to 100.0 -> Provided value ({pct}) might not be correct.");
        }

        if !(0.0..=100.0).contains(&pct) {
            let e = InvalidMarkError::PercentOutOfRange(pct);
            error!("{e}");
            return Err(e);
        }

        Ok(Self::Percent(pct))
    }

    /// Create a **valid** [`Mark::Letter`] with the provided [`char`].
    ///
    /// # Errors
    /// - `c` is **not** within range `A..=Z`
    pub fn letter(c: char) -> MarkResult {
        if !('A'..='Z').contains(&c) {
            let e = InvalidMarkError::LetterOutOfRange(c);
            error!("{e}");
            return Err(e);
        }

        Ok(Self::Letter(c))
    }

    /// Create a **valid** [`Mark::OutOf`] with the provided [`u32`]s.
    ///
    /// # Errors
    /// - `a` is greater than `b`
    pub fn out_of(a: u32, b: u32) -> MarkResult {
        if a > b {
            let e = InvalidMarkError::OutOfTupleEquality(a, b);
            error!("{e}");
            return Err(e);
        }

        Ok(Self::OutOf(a, b))
    }
}

impl Display for Mark {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Percent(pct) => write!(f, "{pct:.2}%"),
            Self::Letter(c) => write!(f, "{c}"),
            Self::OutOf(a, b) => write!(f, "{a} out of {b}"),
        }
    }
}

/// The value contained in the [mark](Mark) is invalid.
#[derive(Error, Debug)]
pub enum InvalidMarkError {
    /// [`Mark::Percent`] value is outside the valid range
    #[error("Mark::Percent: value ({0}) is outside the valid range: 0.0 to 100.0")]
    PercentOutOfRange(f64),
    /// [`Mark::Letter`] char is outside the valid range
    #[error("Mark::Letter: char ({0}) is outside the valid range: A to Z")]
    LetterOutOfRange(char),
    /// [`Mark::OutOf`] left value is greater than right value
    #[error("Mark::OutOf: left value ({0}) is greater than right value ({1})")]
    OutOfTupleEquality(u32, u32),
}

#[cfg(test)]
mod tests {
    use super::Mark::{self, Letter, OutOf, Percent};
    use rstest::rstest;

    #[rstest]
    #[case(Percent(0.0))]
    #[case(Percent(75.0))]
    #[case(Percent(100.0))]
    #[case(Letter('A'))]
    #[case(Letter('E'))]
    #[case(Letter('Z'))]
    #[case(OutOf(0, 0))]
    #[case(OutOf(22, 25))]
    #[case(OutOf(90, 100))]
    #[case(OutOf(1, 1))]
    fn valid(#[case] mark: Mark) {
        assert!(mark.check_valid().is_ok());
    }

    #[rstest]
    #[case(Percent(-0.1))]
    #[case(Percent(100.1))]
    #[case(Letter('a'))]
    #[case(Letter('$'))]
    #[case(Letter('0'))]
    #[case(Letter('%'))]
    #[case(OutOf(1, 0))]
    #[case(OutOf(10, 5))]
    fn invalid(#[case] mark: Mark) {
        assert!(mark.check_valid().is_err());
    }
}
