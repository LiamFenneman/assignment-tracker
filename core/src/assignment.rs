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
    #[error("assignment name was not provided")]
    NoName,
}

impl Assignment {
    pub fn builder() -> AssignmentBuilder {
        AssignmentBuilder::default()
    }

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

#[derive(Debug, Default)]
pub struct AssignmentBuilder {
    name: Option<String>,
    mark: Option<u32>,
    weight: Option<u32>,
    // percentage: calculated when building the assignment
}

impl AssignmentBuilder {
    /// Builds and returns an [Assignment].
    ///
    /// # Errors
    /// `name` is not provided.
    /// `mark` or `weight` is provided and *not* between 0 and 100.
    pub fn build(self) -> Result<Assignment, AssignmentError> {
        let Some(name) = self.name else {
            return Err(AssignmentError::NoName);
        };

        let mut a = Assignment::new(&name);

        if let Some(mark) = self.mark {
            a.set_mark(mark)?;
        }

        if let Some(weight) = self.weight {
            a.set_weight(weight)?;
        }

        return Ok(a);
    }

    /// Provide a name for the [Assignment].
    pub fn name(&mut self, name: &str) -> &mut Self {
        self.name = Some(name.to_owned());
        self
    }

    /// Provide a mark for the [Assignment].
    ///
    /// # Constraints
    /// `mark` must be between 0 and 100.
    ///
    /// Enforcement occurs after calling [AssignmentBuilder::build].
    pub fn mark(&mut self, mark: u32) -> &mut Self {
        self.mark = Some(mark);
        self
    }

    /// Provide a weight for the [Assignment].
    ///
    /// # Constraints
    /// `weight` must be between 0 and 100.
    ///
    /// Enforcement occurs after calling [AssignmentBuilder::build].
    pub fn weight(&mut self, weight: u32) -> &mut Self {
        self.weight = Some(weight);
        self
    }
}
