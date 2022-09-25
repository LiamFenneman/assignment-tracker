use std::fmt;

/// A letter grade.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Grade {
    grade: char,
    modifier: Option<Modifier>,
}

impl Grade {
    /// Create a new grade.
    #[must_use]
    pub fn new(grade: char, modifier: Option<Modifier>) -> Self {
        Grade { grade, modifier }
    }

    /// Get the grade's letter.
    #[must_use]
    pub fn grade(&self) -> char {
        self.grade
    }

    /// Get the grade's modifier.
    #[must_use]
    pub fn modifier(&self) -> Option<Modifier> {
        self.modifier
    }
}

impl fmt::Display for Grade {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.grade)?;
        if let Some(modifier) = self.modifier {
            write!(f, "{}", modifier)?;
        }
        Ok(())
    }
}

/// A letter grade modifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Modifier {
    Plus,
    Minus,
}

impl fmt::Display for Modifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Modifier::Plus => write!(f, "+"),
            Modifier::Minus => write!(f, "-"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case('A', None)]
    #[case('B', Some(Modifier::Plus))]
    #[case('C', Some(Modifier::Minus))]
    fn grade_new(#[case] letter: char, #[case] modifier: Option<Modifier>) {
        let grade = Grade::new(letter, modifier);
        assert_eq!(grade.grade(), letter);
        assert_eq!(grade.modifier(), modifier);
    }

    #[rstest]
    #[case('A', None, "A")]
    #[case('A', Some(Modifier::Plus), "A+")]
    #[case('A', Some(Modifier::Minus), "A-")]
    fn grade_display(
        #[case] grade: char,
        #[case] modifier: Option<Modifier>,
        #[case] expected: &str,
    ) {
        let grade = Grade::new(grade, modifier);
        assert_eq!(grade.to_string(), expected);
    }
}
