use std::{env, error::Error, fs, path::Path, rc::Rc};

use rand::prelude::*;
use tracker_lib::{assignment::InvalidError, Assignment, ClassCode, Tracker};

type Result<T> = std::result::Result<T, Box<dyn Error + 'static>>;

fn main() {
    let tracker = gen_random_tracker();

    let args: Vec<String> = env::args().skip(1).collect();

    let cmd = args
        .get(0)
        .expect("CLI requires at least 1 argument to be passed");

    match cmd {
        _ if cmd == "write" => {
            if let Some(filename) = args.get(1) {
                write_file(&tracker, filename.trim()).unwrap();
            }
        }
        _ => panic!("CLI was passed an unknown argument"),
    }
}

fn write_file(tracker: &Tracker, filename: &str) -> Result<()> {
    let path = Path::new(filename);
    let mut contents = String::new();
    for a in tracker.get_all() {
        contents.push_str(&to_csv(a));
        contents.push('\n');
    }

    if let Err(e) = fs::write(path, contents) {
        return Err(Box::new(e));
    }

    Ok(())
}

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

fn from_csv(csv: &str) -> Result<Tracker> {
    let mut tracker = Tracker::new();

    for line in csv.lines() {
        let vec: Vec<&str> = line.split(",").collect();

        // parse the class code, name, and value
        let code = match tracker.get_code(vec.get(0).expect("Line must have a class code")) {
            Ok(c) => c,
            Err(e) => return Err(Box::new(InvalidError::with_msg(e))),
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
            return Err(Box::new(InvalidError::with_msg(
                "Mark part of CSV must be a number (#.#) or 'None'",
            )));
        }

        // add the assignment to the tracker
        tracker.track(ass)?;
    }

    Ok(tracker)
}

fn gen_random_tracker() -> Tracker {
    let mut tracker = Tracker::new();
    let mut rng = thread_rng();

    let num_assign: u32 = rng.gen_range(5..=20);
    for i in 0..num_assign {
        let code = gen_rand_code(&mut tracker);
        let name = format!("Assignment {}", i + 1);
        let a = Assignment::new(&name, 1.0, code).unwrap();
        tracker.track(a).unwrap();
    }

    tracker
}

fn gen_rand_code(tracker: &mut Tracker) -> Rc<ClassCode> {
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
