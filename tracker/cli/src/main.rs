use std::rc::Rc;

use rand::prelude::*;
use tracker_lib::{Assignment, ClassCode, Tracker};

fn main() {
    let mut tracker = gen_random_tracker();
    println!("get_all()");
    println!("{:#?}\n", tracker.get_all());

    println!("get_all_from_class()");
    let code = tracker.get_code("RAND101").unwrap();
    println!("{:#?}\n", tracker.get_all_from_class(Rc::clone(&code)));

    println!("to_csv()");
    let a1 = tracker.get_all().first().unwrap();
    println!("{}\n", to_csv(a1));
    let mut a2 = Assignment::new("Exam", 20.0, Rc::clone(&code)).unwrap();
    a2.set_mark(90.0).unwrap();
    println!("{}\n", to_csv(&a2));
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
