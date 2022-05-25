use anyhow::Result;
use tracker_core::*;

fn assignment() -> Result<()> {
    let mut a = Assignment::new(0, "Test", 50.0)?;

    a.set_mark(100.0)?;
    a.remove_mark();
    a.remove_mark();
    a.set_mark(-100.0)?;

    Ok(())
}

fn class() -> Result<()> {
    let mut class = Class::new(0, "TEST101")?;
    let a = Assignment::new(0, "Test", 50.0)?;

    let _ = class.add_assignment(a);
    let _ = class.add_mark(0, 75.0);
    let _ = class.add_mark(1, 10.0);
    let _ = class.add_mark(0, -10.0);
    let _ = class.remove_assignment(0);

    Ok(())
}

fn tracker() -> Result<()> {
    let mut tracker = Tracker::default();
    tracker.add_class(Class::new(0, "TEST123")?)?;
    tracker.add_class(Class::new(1, "SOME456")?)?;
    tracker.add_class(Class::new(2, "OTHR789")?)?;
    tracker.track(0, Assignment::new(0, "Test 1", 50.0)?)?;
    tracker.track(0, Assignment::new(1, "Test 2", 50.0)?)?;
    tracker.track(1, Assignment::new(2, "Test 1", 50.0)?)?;
    tracker.track(1, Assignment::new(3, "Test 2", 50.0)?)?;
    tracker.track(2, Assignment::new(4, "Test 1", 50.0)?)?;
    tracker.track(2, Assignment::new(5, "Test 2", 50.0)?)?;
    Ok(())
}

fn main() {
    pretty_env_logger::init();

    println!("Assignment Test:");
    let _ = assignment();

    println!("Class Test:");
    let _ = class();

    println!("Tracker Test:");
    let _ = tracker();
}
