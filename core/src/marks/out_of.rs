#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OutOf {
    mark: u16,
    out_of: u16,
}

impl OutOf {
    /// Create a new out of mark.
    ///
    /// # Errors
    /// - If `mark` is greater than `out_of`.
    pub fn new(mark: u16, out_of: u16) -> Result<Self, Error> {
        if mark > out_of {
            return Err(Error::MarkTooLarge(mark, out_of));
        }
        Ok(OutOf { mark, out_of })
    }

    /// Get the mark.
    #[must_use]
    pub fn mark(&self) -> u16 {
        self.mark
    }

    /// Get the out of.
    #[must_use]
    pub fn out_of(&self) -> u16 {
        self.out_of
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("invalid out of mark: `{0}` < `{1}` must be true")]
    MarkTooLarge(u16, u16),
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(0, 0)]
    #[case(0, 1)]
    #[case(10, 15)]
    #[case(15, 15)]
    #[case(u16::MIN, u16::MIN)]
    #[case(u16::MIN, u16::MAX)]
    #[case(u16::MAX, u16::MAX)]
    fn out_of_new(#[case] mark: u16, #[case] out_of: u16) {
        let r = OutOf::new(mark, out_of).unwrap();
        assert_eq!(r.mark(), mark);
        assert_eq!(r.out_of(), out_of);
    }

    #[rstest]
    #[case(1, 0)]
    #[case(u16::MAX, u16::MIN)]
    fn out_of_new_invalid(#[case] mark: u16, #[case] out_of: u16) {
        let out_of = OutOf::new(mark, out_of);
        assert!(out_of.is_err());
    }
}
