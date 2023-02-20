use crate::Assignment;
use std::collections::VecDeque;
use thiserror::Error;

const MAX_SUM_WEIGHT: u32 = 100;

/// Collection of [Assignment]s.
///
/// Backed by a [VecDeque].
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Assignments {
    inner: VecDeque<Assignment>,
}

#[derive(Debug, Error)]
pub enum AssignmentsError {
    #[error("Assignment with name {0} already exists")]
    NonUniqueName(String),
    #[error("Sum of all assignment weights is out of bounds (found: `{0}`, max: `100`)")]
    WeightsOutOfBounds(u32),
}

impl Assignments {
    /// Creates a new [Assignments] collection.
    pub fn new() -> Self {
        Self { inner: VecDeque::new() }
    }

    /// Returns an [Assignment] at the given index.
    pub fn get(&self, index: usize) -> Option<&Assignment> {
        self.inner.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Assignment> {
        self.inner.get_mut(index)
    }

    /// Returns the length of the collection.
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Appends an element to the back of the collection.
    ///
    /// See [VecDeque::push_back].
    ///
    /// # Errors
    /// An assignment with the same name already exists.
    /// The sum of all assignment weights is out of bounds (`>100`).
    pub fn push_back(&mut self, assignment: Assignment) -> Result<(), AssignmentsError> {
        self.can_add_assignment(&assignment)?;
        self.inner.push_back(assignment);
        Ok(())
    }

    /// Appends an element to the front of the collection.
    ///
    /// See [VecDeque::push_front].
    ///
    /// # Errors
    /// An assignment with the same name already exists.
    /// The sum of all assignment weights is out of bounds (`>100`).
    pub fn push_front(&mut self, assignment: Assignment) -> Result<(), AssignmentsError> {
        self.can_add_assignment(&assignment)?;
        self.inner.push_front(assignment);
        Ok(())
    }

    /// Pops an element from the back of the collection.
    ///
    /// See [VecDeque::pop_back].
    pub fn pop_back(&mut self) -> Option<Assignment> {
        self.inner.pop_back()
    }

    /// Pops an element from the front of the collection.
    ///
    /// See [VecDeque::pop_front].
    pub fn pop_front(&mut self) -> Option<Assignment> {
        self.inner.pop_front()
    }

    /// Removes an element from the collection at the given index.
    ///
    /// See [VecDeque::remove].
    pub fn remove(&mut self, index: usize) -> Option<Assignment> {
        self.inner.remove(index)
    }

    /// Moves all the [Assignment]s of `other` into `self`, leaving `other` empty.
    ///
    /// See [VecDeque::append].
    pub fn append(&mut self, other: &mut Self) {
        self.inner.append(&mut other.inner);
    }

    /// Check if the assignment is allowed to be added to the collection.
    fn can_add_assignment(&self, assignment: &Assignment) -> Result<(), AssignmentsError> {
        if self.inner.iter().any(|a| a.name() == assignment.name()) {
            return Err(AssignmentsError::NonUniqueName(assignment.name().to_owned()));
        }

        let sum = self.inner.iter().filter_map(|a| a.weight()).sum::<u32>();
        if sum > MAX_SUM_WEIGHT {
            return Err(AssignmentsError::WeightsOutOfBounds(sum));
        }

        Ok(())
    }
}

impl IntoIterator for Assignments {
    type Item = Assignment;

    type IntoIter = std::collections::vec_deque::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'a> Extend<&'a Assignment> for Assignments {
    fn extend<T: IntoIterator<Item = &'a Assignment>>(&mut self, iter: T) {
        self.inner.extend(iter.into_iter().cloned())
    }
}

impl<const N: usize> From<[Assignment; N]> for Assignments {
    fn from(assignments: [Assignment; N]) -> Self {
        Self {
            inner: VecDeque::from(assignments),
        }
    }
}

impl From<Vec<Assignment>> for Assignments {
    fn from(assignments: Vec<Assignment>) -> Self {
        Self {
            inner: VecDeque::from(assignments),
        }
    }
}
