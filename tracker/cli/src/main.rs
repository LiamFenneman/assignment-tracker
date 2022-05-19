#[macro_use]
extern crate prettytable;

use prettytable::{Cell, Row, Table};
use rand::prelude::*;
use std::{
    env,
    error::Error,
    fs,
    io::{self, Write},
    path::Path,
    process,
    rc::Rc,
};
use tracker_core::{assignment::InvalidError, Assignment, ClassCode, Tracker};

type Result<T> = std::result::Result<T, Box<dyn Error + 'static>>;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let filename = args.get(0).expect("A filename (or path) must be provided");
    let mut tracker = read_file(filename).expect("Problem finding the given filename");

    println!("Enter command or help to get a list of commands");

    loop {
        let input: Vec<String> = get_input()
            .unwrap_or_else(|e| {
                eprintln!("Problem getting input: {}", e);
                process::exit(1);
            })
            .split(' ')
            .map(|e| e.to_owned())
            .collect();

        let cmd = input.get(0);
        let args: Vec<String> = input.iter().skip(1).map(|e| e.to_owned()).collect();
        match cmd {
            None => break,
            Some(c) => {
                if let Err(e) = do_command(c, &args, &mut tracker) {
                    eprintln!("{}", e);
                    break;
                }
            }
        }
    }
}

/// Execute a command based on `cmd` using the `args` and [`tracker`](Tracker).
fn do_command(cmd: &str, args: &[String], tracker: &mut Tracker) -> Result<()> {
    match cmd {
        _ if cmd == "help" => {
            ptable!(
                ["COMMAND", "ARGUMENTS", "DESCRIPTION"],
                ["help", "", "print this message"],
                ["write", "<filename>", "write the tracker to the given file"],
                ["print", "", "print a table of all assignments"]
            );
        }
        _ if cmd == "write" => {
            if let Some(filename) = args.get(0) {
                println!("Writing to {}...", filename);
                write_file(tracker, filename).unwrap();
            }
        }
        _ if cmd == "print" => {
            print_table(tracker);
        }
        _ => panic!("CLI was passed an unknown argument"),
    }

    Ok(())
}

/// Get user input.
///
/// Print "`> `" to the console to indicate the user can enter commands.
/// Then retrieve the user input and trim the line.
fn get_input() -> Result<String> {
    print!("> ");
    io::stdout().flush()?;
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_owned())
}

/// Print all assignments in the tracker to ```stdout```.
fn print_table(tracker: &Tracker) {
    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("CLASS CODE"),
        Cell::new("NAME"),
        Cell::new("MARK"),
        Cell::new("VALUE"),
        Cell::new("FINAL PCT"),
    ]));

    for ass in tracker.get_all() {
        let mark_str = match ass.mark() {
            Some(m) => format!("{:.1}", m),
            None => "No mark".to_owned(),
        };
        let pct_str = match ass.final_pct() {
            Some(m) => format!("{:.1}", m),
            None => String::new(),
        };
        table.add_row(Row::new(vec![
            Cell::new(&format!("{}", ass.class_code())),
            Cell::new(ass.name()),
            Cell::new(&mark_str),
            Cell::new(&format!("{:.1}", ass.value())),
            Cell::new(&pct_str),
        ]));
    }

    table.printstd();
}

/// Write all assignments from Tracker into a file.
fn write_file(tracker: &Tracker, filename: &str) -> Result<()> {
    let mut contents = String::new();
    for a in tracker.get_all() {
        contents.push_str(&to_csv(a));
        contents.push('\n');
    }

    if let Err(e) = fs::write(Path::new(filename), contents) {
        return Err(Box::new(e));
    }

    Ok(())
}

/// Read from the given file and create a Tracker.
fn read_file(filename: &str) -> Result<Tracker> {
    let contents = fs::read_to_string(Path::new(filename))?;
    let tracker = from_csv(&contents)?;
    Ok(tracker)
}

/// Convert an assignment into CSV.
fn to_csv(ass: &Assignment) -> String {
    match ass.mark() {
        Some(m) => format!(
            "{},{},{:.1},{:.1}",
            ass.class_code(),
            ass.name(),
            m,
            ass.value()
        ),
        None => format!(
            "{},{},None,{:.1}",
            ass.class_code(),
            ass.name(),
            ass.value()
        ),
    }
}

/// Convert CSV into a Tracker.
fn from_csv(csv: &str) -> Result<Tracker> {
    let mut tracker = Tracker::new();

    for line in csv.lines() {
        let vec: Vec<&str> = line.split(',').collect();

        // parse the class code, name, and value
        let code = match tracker.get_code(vec.get(0).expect("Line must have a class code")) {
            Ok(c) => c,
            Err(e) => return Err(Box::new(InvalidError(e))),
        };
        let name: &str = vec.get(1).expect("Line must have a name");
        let value: f64 = vec.get(3).expect("Line must have a value").parse()?;

        // create the assignment
        let mut ass = Assignment::new(name, value, code)?;

        // add the mark if there is one
        let v2 = vec.get(2).expect("Line must have a mark or None");
        if let Ok(mark) = v2.parse() {
            ass.set_mark(mark)?;
        } else if *v2 != "None" {
            // if a number can't be parsed then it must be None
            return Err(Box::new(InvalidError(
                "Mark part of CSV must be a number (#.#) or 'None'",
            )));
        }

        // add the assignment to the tracker
        tracker.track(ass)?;
    }

    Ok(tracker)
}

/// Generate a tracker and populate it with random assignments.
fn _gen_rand_tracker() -> Tracker {
    let mut tracker = Tracker::new();
    let mut rng = thread_rng();

    let num_assign: u32 = rng.gen_range(5..=20);
    for i in 0..num_assign {
        let code = _gen_rand_code(&mut tracker);
        let name = format!("Assignment {}", i + 1);
        let a = Assignment::new(&name, 1.0, code).unwrap();
        tracker.track(a).unwrap();
    }

    tracker
}

/// Generate a random class code.
fn _gen_rand_code(tracker: &mut Tracker) -> Rc<ClassCode> {
    let i: u32 = thread_rng().gen_range(2..=5);
    let code = format!("RAND10{}", i - 2);
    tracker.get_code(&code).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_csv_valid() {
        let csv = "RAND100,Assignment 1,None,1.0\r\nRAND101,Exam,90.0,20.0";
        let tracker = from_csv(csv).unwrap();
        for (i, line) in csv.lines().enumerate() {
            assert_eq!(line, to_csv(tracker.get_all().get(i).unwrap()))
        }
    }

    #[test]
    fn from_csv_valid_1() {
        let csv = "RAND100,Assignment 1,None,1.0\r\nRAND101,Exam,90.0,20.0";
        let tracker = from_csv(csv);
        assert!(tracker.is_ok());
        assert_eq!(2, tracker.unwrap().get_all().len());
    }

    #[test]
    fn from_csv_valid_2() {
        let csv = "";
        assert_eq!(Tracker::new(), from_csv(csv).unwrap());
    }

    #[test]
    fn from_csv_invalid_1() {
        let csv = ",,,";
        assert!(from_csv(csv).is_err());
    }

    #[test]
    fn from_csv_invalid_2() {
        let csv = "\r\n\r\n\r\n";
        assert!(from_csv(csv).is_err());
    }

    #[test]
    fn from_csv_invalid_3() {
        let csv = "something,is,here,?";
        assert!(from_csv(csv).is_err());
    }

    #[test]
    fn from_csv_invalid_4() {
        let csv = "RAND100,Assignment 1,NONE,1.0";
        assert!(from_csv(csv).is_err());
    }
}
