use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};
use thiserror::Error;

/// Generic representation a university/school class/paper/course.
pub trait Classlike: Display + Debug + PartialEq + PartialOrd {
    /// A **unique** *code* for a [class](Classlike).
    ///
    /// **Uniqueness** is enforced by the **user** of the [class](Classlike).
    ///
    /// The *code* should only allow any number of ASCII letters (a-z, A-Z) and numbers (0-9).
    fn code(&self) -> &str;

    /// The **full** name of a [class](Classlike).
    ///
    /// **Full** only refers to the fact that this is a **cosmetic** value that *can* allow use of UTF-8.
    fn name(&self) -> &str;

    /// Create a new [class](Classlike) where the *code* and *name* are the same.
    fn new(code: &str) -> Self;

    /// Create a new [class](Classlike) where the *code* and *name* are the different.
    fn with_name(code: &str, name: &str) -> Self;

    /// Total value of the class.
    ///
    /// # Ensures
    /// Total value is within `0.0..=100.0`
    fn total_value(&self) -> f64;

    /// Set the total value of the [assignments](crate::prelude::Assignmentlike) within the [class](Classlike).
    ///
    /// # Errors
    /// - `value` is **not** within the range `0.0..=100.0`
    fn set_total_value(&mut self, value: f64) -> Result<()>;

    /// Add to the total value of the [assignments](crate::prelude::Assignmentlike) within the [class](Classlike).
    ///
    /// # Errors
    /// - `total_value() + to_add` is **not** within the range `0.0..=100.0`
    fn add_total_value(&mut self, to_add: f64) -> Result<()> {
        self.set_total_value(self.total_value() + to_add)
    }

    /// Remove from the total value of the [assignments](crate::prelude::Assignmentlike) within the [class](Classlike).
    ///
    /// # Errors
    /// - `total_value() - to_remove` is **not** within the range `0.0..=100.0`
    fn remove_total_value(&mut self, to_remove: f64) -> Result<()> {
        self.set_total_value(self.total_value() - to_remove)
    }
}

/// Implementation of [Classlike] that **only** contains a *code*.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Code {
    code: String,
    total_value: f64,
}

impl Classlike for Code {
    fn code(&self) -> &str {
        &self.code
    }

    fn name(&self) -> &str {
        self.code()
    }

    fn new(code: &str) -> Self {
        Self {
            code: code.to_owned(),
            total_value: 0.0,
        }
    }

    fn with_name(code: &str, _name: &str) -> Self {
        Self::new(code)
    }

    fn total_value(&self) -> f64 {
        self.total_value
    }

    fn set_total_value(&mut self, value: f64) -> Result<()> {
        if !(0.0..=100.0).contains(&value) {
            bail!(InvalidClassError::TotalValueOutOfRange(
                self.name().to_owned(),
                value
            ));
        }

        trace!("{self} -> Total value -> {value}");
        self.total_value = value;
        Ok(())
    }
}

impl Display for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.code())
    }
}

impl Default for Code {
    fn default() -> Self {
        Self::new("DEFAULT")
    }
}

/// Implementation of [Classlike] that contains a *code* **and** a *name*.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Class {
    code: String,
    name: String,
    total_value: f64,
}

impl Classlike for Class {
    fn code(&self) -> &str {
        &self.code
    }

    fn name(&self) -> &str {
        self.code()
    }

    fn new(code: &str) -> Self {
        Self::with_name(code, code)
    }

    fn with_name(code: &str, name: &str) -> Self {
        Self {
            code: code.to_owned(),
            name: name.to_owned(),
            total_value: 0.0,
        }
    }

    fn total_value(&self) -> f64 {
        self.total_value
    }

    fn set_total_value(&mut self, value: f64) -> Result<()> {
        if !(0.0..=100.0).contains(&value) {
            bail!(InvalidClassError::TotalValueOutOfRange(
                self.name().to_owned(),
                value
            ));
        }

        trace!("{self} -> Total value -> {value}");
        self.total_value = value;
        Ok(())
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name(), self.code())
    }
}

impl Default for Class {
    fn default() -> Self {
        Self::with_name("DEFAULT", "Default Class")
    }
}

/// The [class](Classlike) is invalid.
#[derive(Error, Debug)]
pub enum InvalidClassError {
    /// The total value of all assignments must be within `0.0..=100.0`
    #[error("{0} -> Total value ({1}) must be within 0.0..=100.0")]
    TotalValueOutOfRange(String, f64),
}
