use thiserror::Error;

/// Representation of an [Assignment].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Assignment {
    name: String,
    mark: Option<u32>,
    weight: Option<u32>,
    percentage: Option<u32>,
}

#[derive(Error, Debug)]
pub enum AssignmentError {
    #[error("the Value `{0}` is not within a percentage range")]
    NotPercentage(u32),
}

impl Assignment {
    /// Create a new [Assignment] with a name.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            ..Default::default()
        }
    }

    /// Get the name of the [Assignment].
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the mark as a percentage.
    pub fn mark(&self) -> Option<u32> {
        self.mark
    }

    /// Get the weight for contribution to final grade as a percentage.
    pub fn weight(&self) -> Option<u32> {
        self.weight
    }

    /// Get the final grade as a percentage.
    pub fn percentage(&self) -> Option<u32> {
        self.percentage
    }

    /// Set the mark for the [Assignment].
    /// 
    /// # Errors
    /// - `mark` is greater than 100.
    pub fn set_mark(&mut self, mark: u32) -> Result<(), AssignmentError> {
        if mark > 100 {
            return Err(AssignmentError::NotPercentage(mark));
        }
        self.mark = Some(mark);
        self.update_percentage();
        Ok(())
    }

    /// Set the weight for the [Assignment].
    /// 
    /// # Errors
    /// - `weight` is greater than 100.
    pub fn set_weight(&mut self, weight: u32) -> Result<(), AssignmentError> {
        if weight > 100 {
            return Err(AssignmentError::NotPercentage(weight));
        }
        self.weight = Some(weight);
        self.update_percentage();
        Ok(())
    }

    fn update_percentage(&mut self) {
        if let (Some(mark), Some(weight)) = (self.mark, self.weight) {
            self.percentage = Some(mark * weight / 100);
        }
    }
}

impl Default for Assignment {
    fn default() -> Self {
        Self {
            name: String::from("Unknown assignment"),
            mark: None,
            weight: None,
            percentage: None,
        }
    }
}
