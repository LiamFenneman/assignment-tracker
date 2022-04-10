mod utils;

use std::{error::Error, fs, io, process};
pub use utils::{Args, Assignment};

/// Entry point to the program
pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    // read the file from the args
    let contents = fs::read_to_string(args.filename())?;
    // println!("{} contents:", args.filename());
    // for line in contents.lines() {
    //     println!("{}", line)
    // }

    for line in contents.lines() {
        let assign: Assignment = line.parse().unwrap_or_else(|err| {
            eprintln!("{}", err);
            process::exit(1);
        });
        println!("{}", assign);
    }

    // create an assignment
    // let mut a = Assignment::new("Test", 50.0, "SOME101").unwrap();
    // println!("New assignment: {}", a);
    // if let Err(e) = a.set_mark(88.0) {
    //     eprintln!("Assignment error: {}", e);
    //     process::exit(1);
    // }
    // println!("With mark: {}", a);

    // get user input
    let input = get_input("Command or exit (q)").unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    // exit if the user types "q", "quit" or "exit"
    if input == "q" || input == "quit" || input == "exit" {
        process::exit(0);
    }

    Ok(())
}

/// Ask the user for input.
fn get_input(msg: &str) -> Result<String, io::Error> {
    println!("{}:", msg);
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_string())
}
