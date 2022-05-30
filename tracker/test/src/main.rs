use anyhow::Result;
use chrono::NaiveDate;
use rand::{thread_rng, Rng};
use tracker_core::*;

fn invalid_marks() {
    let _ = Mark::percent(-10.0);
    let _ = Mark::percent(110.0);
    let _ = Mark::letter('a');
    let _ = Mark::letter('$');
    let _ = Mark::letter('1');
    let _ = Mark::out_of(1, 0);
    let _ = Mark::out_of(21, 20);
}

fn assignment() -> Result<()> {
    let mut a = Assignment::new(0, "Exam", 25.0);
    a.set_mark(Mark::percent(75.0)?)?;
    a.set_mark(Mark::letter('A')?)?;
    a.set_mark(Mark::out_of(22, 25)?)?;
    a.set_due_date(NaiveDate::from_ymd(2022, 5, 1).and_hms(23, 59, 0));
    Ok(())
}

fn assignment_builder() -> Result<()> {
    let _ = Assignment::builder(2, "Test", 15.0).build();
    let _ = Assignment::builder(0, "Exam", 25.0)
        .mark(Mark::percent(25.0)?)
        .build();
    let _ = Assignment::builder(10, "Test", 50.0)
        .mark(Mark::letter('A')?)
        .build();
    let _ = Assignment::builder(555, "Assignment", 10.0)
        .mark(Mark::out_of(22, 25)?)
        .due_date(NaiveDate::from_ymd(2022, 5, 1).and_hms(15, 24, 55))
        .build();
    let _ = Assignment::builder(555, "Assignment", 10.0)
        .due_date(NaiveDate::from_ymd(2022, 5, 1).and_hms(15, 24, 55))
        .build();
    Ok(())
}

fn tracker() -> Result<()> {
    const CLASS_A: &str = "CLASS 111";
    const CLASS_B: &str = "OTHER 999";
    const N: u32 = 5;

    let mut t = Tracker::<Code>::new("Code Tracker");

    // CREATE & ADD CLASSES
    t.add_class(Code::new(CLASS_A))?;
    t.add_class(Code::new(CLASS_B))?;

    // CREATE & ADD ASSIGNMENTS
    for i in 0..N {
        t.add_assignment(CLASS_A, gen(i, i, 100.0 / N as f64))?;
        t.add_assignment(CLASS_B, gen(i + N, i, 100.0 / N as f64))?;
    }

    // EDIT ASSIGNMENTS
    t.get_assignment_by_id_mut(1)
        .unwrap()
        .set_mark(Mark::percent(75.0)?)?;

    t.get_assignment_by_id_mut(2)
        .unwrap()
        .set_due_date(NaiveDate::from_ymd(2022, 5, 1).and_hms(23, 59, 59));

    // REMOVE ASSIGNMENTS
    t.remove_assignment(1)?;
    t.remove_assignment(2)?;
    t.remove_assignment(3)?;

    // REMOVE CLASSES
    t.remove_class(CLASS_A)?;
    t.remove_class(CLASS_B)?;

    println!("{t:#?}");

    Ok(())
}

fn gen(a: u32, b: u32, max_v: f64) -> Assignment {
    let mut rng = thread_rng();
    let v = rng.gen_range(0.0..=max_v).round();
    Assignment::new(a, &format!("Assign {b}"), v)
}

fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "trace");
    std::env::set_var("RUST_BACKTRACE", "1");
    pretty_env_logger::init();

    println!("Invalid Mark Test:");
    invalid_marks();

    println!("Assignment Test:");
    assignment()?;

    println!("Assignment Builder Test:");
    assignment_builder()?;

    println!("Tracker Test:");
    let _ = tracker();

    Ok(())
}
