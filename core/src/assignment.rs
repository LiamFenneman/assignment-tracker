/// Representation of an [Assignment].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Assignment {
    pub name: String,
    pub mark: Option<u32>,
    pub weight: Option<u32>,
}

impl Assignment {
    /// Create a new [Assignment] with a name.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            ..Default::default()
        }
    }

    /// Builder: add a mark to the assignment.
    pub fn mark(mut self, mark: u32) -> Self {
        self.mark = Some(mark);
        self
    }

    /// Builder: add a weight to the assignment.
    pub fn weight(mut self, weight: u32) -> Self {
        self.weight = Some(weight);
        self
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
