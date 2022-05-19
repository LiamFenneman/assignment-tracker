use crate::Class;
use log::trace;

/// Maximum length of the name of an [assignment](Assignment) in bytes.
pub const MAX_NAME_LEN: usize = 48;

/// A single assignment.
#[derive(Debug, Clone, PartialEq)]
pub struct Assignment<'c> {
    id: u64,
    name: String,
    mark: Option<f64>,
    value: f64,
    class: Option<&'c Class>,
}

impl<'c> Assignment<'c> {
    /// Create a new [assignment](Assignment) using the [builder](AssignmentBuilder) pattern.
    pub fn builder(id: u64) -> AssignmentBuilder<'c> {
        AssignmentBuilder {
            id,
            name: None,
            mark: None,
            value: None,
            class: None,
        }
    }
}

pub struct AssignmentBuilder<'c> {
    id: u64,
    name: Option<String>,
    mark: Option<f64>,
    value: Option<f64>,
    class: Option<&'c Class>,
}

impl<'c> AssignmentBuilder<'c> {
    /// Build an [Assignment].
    pub fn build(&mut self) -> Assignment<'c> {
        let a = Assignment {
            id: self.id,
            name: self.name.to_owned().expect("A name must be provided."),
            mark: self.mark,
            value: self.value.expect("A value must be provided."),
            class: self.class,
        };

        trace!("Built assignment: {a:?}");

        a
    }

    /// Set the name of the [assignment](Assignment).
    ///
    /// # Panics
    /// - `name` is empty
    /// - `name` is longer than [MAX_NAME_LEN]
    pub fn name(&mut self, name: &str) -> &mut Self {
        if name.is_empty() {
            panic!("Name must be provided.");
        }

        if name.len() > MAX_NAME_LEN {
            panic!("Name must be less than {MAX_NAME_LEN} bytes.");
        }

        self.name = Some(name.to_owned());
        self
    }

    /// Set the value of the [assignment](Assignment).
    ///
    /// # Panics
    /// - `value` outside the range `0.0..=100.0`
    pub fn value(&mut self, value: f64) -> &mut Self {
        if !(0.0..=100.0).contains(&value) {
            panic!("Value must be within 0.0 and 100.0");
        }

        self.value = Some(value);
        self
    }

    /// Set the mark of the [assignment](Assignment).
    ///
    /// # Panics
    /// - `mark` outside the range `0.0..=100.0`
    pub fn mark(&mut self, mark: Option<f64>) -> &mut Self {
        if let Some(m) = mark {
            if !(0.0..=100.0).contains(&m) {
                panic!("Mark must be within 0.0 and 100.0");
            }
        }

        self.mark = mark;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("Assignment 1", Some(90.0), 30.0)]
    #[case("Assignment 2", None, 30.0)]
    #[should_panic]
    #[case("", None, 30.0)]
    #[should_panic]
    #[case("Some super long assignment name which is too long", None, 30.0)]
    #[should_panic]
    #[case("Test", Some(-1.0), 30.0)]
    #[should_panic]
    #[case("Test", Some(101.0), 30.0)]
    #[should_panic]
    #[case("Test", Some(90.0), -1.0)]
    #[should_panic]
    #[case("Test", Some(90.0), 100.1)]
    fn build(#[case] name: &str, #[case] mark: Option<f64>, #[case] value: f64) {
        let a = Assignment::builder(0)
            .name(name)
            .value(value)
            .mark(mark)
            .build();

        assert_eq!(
            Assignment {
                id: 0,
                name: name.to_owned(),
                mark,
                value,
                class: None
            },
            a
        );
    }
}
