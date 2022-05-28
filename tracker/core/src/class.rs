use std::fmt::{Debug, Display};
/// Generic representation a university/school class/paper/course.
pub trait Classlike: Display + Debug + PartialEq + Eq + PartialOrd + Ord {
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
}

/// Implementation of [Classlike] that **only** contains a *code*.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Code {
    code: String,
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
        }
    }

    fn with_name(code: &str, _name: &str) -> Self {
        Self {
            code: code.to_owned(),
        }
    }
}

impl Display for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.code())
    }
}

impl Default for Code {
    fn default() -> Self {
        Self {
            code: String::from("DEFAULT"),
        }
    }
}

/// Implementation of [Classlike] that contains a *code* **and** a *name*.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Class {
    code: String,
    name: String,
}

impl Classlike for Class {
    fn code(&self) -> &str {
        &self.code
    }

    fn name(&self) -> &str {
        self.code()
    }

    fn new(code: &str) -> Self {
        Self {
            code: code.to_owned(),
            name: code.to_owned(),
        }
    }

    fn with_name(code: &str, name: &str) -> Self {
        Self {
            code: code.to_owned(),
            name: name.to_owned(),
        }
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name(), self.code())
    }
}

impl Default for Class {
    fn default() -> Self {
        Self {
            code: String::from("DEFAULT"),
            name: String::from("Default Class"),
        }
    }
}
