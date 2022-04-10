mod utils;

use std::{error::Error, fs, io, process};
pub use utils::{Args, Assignment, Class};

/// Entry point to the program
pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    // read the file from the args
    let contents = fs::read_to_string(args.filename())?;
    println!("{} contents:", args.filename());
    for line in contents.lines() {
        println!("{}", line)
    }

    // get user input
    let input = get_input("Write something").unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });
    println!("You wrote: {}", input);

    // create a class
    let c = Class::new("NWEN303").unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });
    println!("New class: {}", c);

    // create an assignment
    let mut a = Assignment::new("Test", 50.0, c);
    println!("New assignment: {}", a);
    if let Err(e) = a.set_mark(Some(88.0)) {
        eprintln!("Assignment error: {}", e);
        process::exit(1);
    }
    println!("With mark: {}", a);

    Ok(())
}

/// Ask the user for input.
fn get_input(msg: &str) -> Result<String, io::Error> {
    println!("{}:", msg);
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_string())
}

#[cfg(test)]
mod tests {}
