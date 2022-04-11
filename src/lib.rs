#[macro_use]
extern crate lazy_static;

mod assignment;
mod utils;
pub use assignment::{Assignment, InvalidError};
pub use utils::Args;

use serde::{Deserialize, Serialize};
use std::{error::Error, fs, io};

/// Entry point to the program
pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let assignments = get_assignments_from_file(args.filename())?;
    println!("{:#?}", assignments);

    Ok(())
}

/// Ask the user for input.
fn _get_input(msg: &str) -> Result<String, io::Error> {
    println!("{}:", msg);
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_string())
}

fn get_assignments_from_file(filename: &str) -> Result<Vec<Assignment>, Box<dyn Error + 'static>> {
    let contents = fs::read_to_string(filename)?;

    let assignments: AssignmentVec =
        serde_json::from_str(&contents).expect("Problem deserialising the file contents");

    if let Err(e) = assignments.is_valid() {
        return Err(Box::new(e));
    }

    Ok(assignments.0) // unwrap from AssignmentVec to Vec<Assignment>
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct AssignmentVec(Vec<Assignment>);

impl AssignmentVec {
    fn is_valid(&self) -> Result<(), InvalidError> {
        for a in &self.0 {
            if let Err(e) = a.is_valid() {
                return Err(e);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        let mut a1 = Assignment::new("Test 1", 25.0, "TEST101").unwrap();
        a1.set_mark(90.0).unwrap();
        let mut a2 = Assignment::new("Test 2", 25.0, "TEST101").unwrap();
        a2.set_mark(75.0).unwrap();

        let assignments = AssignmentVec(vec![
            a1,
            a2,
            Assignment::new("Test 3", 25.0, "TEST101").unwrap(),
            Assignment::new("Test 4", 25.0, "TEST101").unwrap(),
        ]);

        let serialized = serde_json::to_string(&assignments).unwrap();
        let deserialized: AssignmentVec = serde_json::from_str(&serialized).unwrap();
        assert_eq!(assignments, deserialized)
    }

    #[test]
    fn serialize_valid() {
        test_serialize("data/valid.json", true);
    }

    #[test]
    fn serialize_invalid() {
        test_serialize("data/invalid.json", false);
    }

    fn test_serialize(filename: &str, is_ok: bool) {
        let contents = fs::read_to_string(filename).unwrap();
        let assignments: AssignmentVec = serde_json::from_str(&contents).unwrap();
        if is_ok {
            assert!(assignments.is_valid().is_ok());
        } else {
            assert!(assignments.is_valid().is_err());
        }
    }
}
