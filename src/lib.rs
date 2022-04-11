mod assignment;
mod utils;

pub use assignment::Assignment;
use std::{error::Error, fs, io, process};
pub use utils::Args;

/// Entry point to the program
pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    // read the file from the args
    let contents = fs::read_to_string(args.filename())?;

    let mut assignments: Vec<Assignment> = Vec::with_capacity(contents.len());

    // loop over each line and parse it into an Assignment
    for line in contents.lines() {
        let assign: Assignment = line.parse().unwrap_or_else(|err| {
            eprintln!("{}", err);
            process::exit(1);
        });
        assignments.push(assign);
    }

    println!("{:?}", assignments);

    Ok(())
}

/// Ask the user for input.
fn get_input(msg: &str) -> Result<String, io::Error> {
    println!("{}:", msg);
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_string())
}
