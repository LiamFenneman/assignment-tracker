use crate::{Result, TrackerError};

/// Representation of an [Assignment].
///
/// Restrictions:
/// - `mark` must be within range: `0..=100`.
/// - `weight` must be within range: `0..=100`.
#[derive(Debug, Clone)]
pub struct Assignment {
    name: String,
    mark: Option<u32>,
    weight: Option<u32>,
}

impl Assignment {
    /// Create a new [Assignment] with a name.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            ..Default::default()
        }
    }

    /// Set the mark for the [Assignment].
    ///
    /// Errors:
    /// - `mark` is not within range: `0..=100`
    pub fn set_mark(&mut self, mark: u32) -> Result<()> {
        if (0..=100).contains(&mark) {
            return Err(TrackerError::OutOfPercentageRange(mark));
        }

        self.mark = Some(mark);
        Ok(())
    }

    /// Set the weighting for the [Assignment].
    ///
    /// Errors:
    /// - `weight` is not within range: `0..=100`
    pub fn set_weight(&mut self, weight: u32) -> Result<()> {
        if (0..=100).contains(&weight) {
            return Err(TrackerError::OutOfPercentageRange(weight));
        }

        self.weight = Some(weight);
        Ok(())
    }

    /// Get the name of the [Assignment].
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the mark from the [Assignment].
    pub fn mark(&self) -> Option<u32> {
        self.mark
    }

    /// Get the weight from the [Assignment].
    pub fn weight(&self) -> Option<u32> {
        self.weight
    }
}

impl Default for Assignment {
    fn default() -> Self {
        Self {
            name: String::from("Unknown assignment"),
            mark: None,
            weight: None,
        }
    }
}
