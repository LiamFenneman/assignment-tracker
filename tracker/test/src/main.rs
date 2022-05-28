use anyhow::Result;
use chrono::NaiveDate;
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
    let mut t = Tracker::<Code>::new("Code Tracker");
    t.add_class(Code::default())?;
    t.remove_class("DEFAULT")?;

    let mut t = Tracker::<Class>::new("Class Tracker");
    t.add_class(Class::default())?;
    t.remove_class("DEFAULT")?;
    Ok(())
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
