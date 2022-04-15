mod args;
pub use args::Args;
pub use tracker_lib::Assignment;

use serde::{Deserialize, Serialize};
use std::{env, error::Error, fs, io, path::Path, process, result};
use tracker_lib::ClassCode;

type Result<T> = result::Result<T, Box<dyn Error + 'static>>;

/// Entry point to the program
fn main() -> Result<()> {
    let args = Args::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let mut ass = get_from_file(args.filename())?;

    println!("{:#?}", ass);

    println!("Lets create a new Assignment!");

    // create a new assignment from user input
    let name = get_input("Name")?;
    let code = get_input("Class code (format: XXXX####)")?;
    let value: f64 = get_input("Value (0.0 to 100.0)")?.parse()?;
    let mut assign = Assignment::new(&name, value, ClassCode::new(&code)?)?;
    if let Ok(m) = get_input("Mark (0.0 to 100 OR None)")?.parse() {
        assign.set_mark(m)?;
    };
    ass.push(assign); // add the new assignment to the vec

    // output the vec to a file
    let filename = get_input("File to output (.json)")?;
    write_to_file_pretty(&filename, ass)?;

    Ok(())
}

/// Ask the user for input.
pub fn get_input(msg: &str) -> io::Result<String> {
    println!("{}:", msg);
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_string())
}

/// Get assignments from the given file.
pub fn get_from_file(filename: &str) -> Result<Vec<Assignment>> {
    let contents = fs::read_to_string(filename)?;

    let assignments: AssignmentVec =
        serde_json::from_str(&contents).expect("Problem deserialising the file contents");

    if let Err(e) = assignments.is_valid() {
        return Err(e);
    }

    Ok(assignments.0) // unwrap from AssignmentVec to Vec<Assignment>
}

/// Write the given assignments to a file.
pub fn write_to_file(filename: &str, vec: Vec<Assignment>) -> Result<()> {
    let path = Path::new(filename);
    let contents = serde_json::to_string(&vec)?;

    if let Err(e) = fs::write(path, contents) {
        return Err(Box::new(e));
    }

    Ok(())
}

/// Write the given assignments to a file using pretty format.
pub fn write_to_file_pretty(filename: &str, vec: Vec<Assignment>) -> Result<()> {
    let path = Path::new(filename);
    let contents = serde_json::to_string_pretty(&vec)?;

    if let Err(e) = fs::write(path, contents) {
        return Err(Box::new(e));
    }

    Ok(())
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct AssignmentVec(Vec<Assignment>);

impl AssignmentVec {
    fn is_valid(&self) -> Result<()> {
        for a in &self.0 {
            if let Err(e) = a.is_valid() {
                return Err(Box::new(e));
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
        let mut a1 = Assignment::new("Test 1", 25.0, ClassCode::new("TEST101").unwrap()).unwrap();
        a1.set_mark(90.0).unwrap();
        let mut a2 = Assignment::new("Test 2", 25.0, ClassCode::new("TEST101").unwrap()).unwrap();
        a2.set_mark(75.0).unwrap();

        let assignments = AssignmentVec(vec![
            a1,
            a2,
            Assignment::new("Test 3", 25.0, ClassCode::new("TEST101").unwrap()).unwrap(),
            Assignment::new("Test 4", 25.0, ClassCode::new("TEST101").unwrap()).unwrap(),
        ]);

        let serialized = serde_json::to_string(&assignments).unwrap();
        let deserialized: AssignmentVec = serde_json::from_str(&serialized).unwrap();
        assert_eq!(assignments, deserialized)
    }
}
