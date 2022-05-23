use anyhow::Result;
use pretty_env_logger;
use tracker_core::*;

fn assignment() -> Result<()> {
    let mut a = Assignment::builder(0) //
        .name("Test")
        .value(50.0)
        .build();

    a.set_mark(100.0)?;
    a.remove_mark();
    a.remove_mark();
    a.set_mark(-100.0)?;

    Ok(())
}

fn class() -> Result<()> {
    let mut class = Class::new(0, "TEST101");
    let a = Assignment::builder(0).name("Test").value(50.0).build();

    let _ = class.add_assignment(a);
    let _ = class.add_mark(0, 75.0);
    let _ = class.add_mark(1, 10.0);
    let _ = class.add_mark(0, -10.0);
    let _ = class.remove_assignment(0);

    Ok(())
}

fn tracker() -> Result<()> {
    let mut tracker = Tracker::default();
    tracker.track_class(Class::new(0, "TEST123"))?;
    tracker.track_class(Class::new(1, "SOME456"))?;
    tracker.track_class(Class::new(2, "OTHR789"))?;
    tracker.track_assignment(0, Assignment::builder(0).name("Test 1").value(50.0).build())?;
    tracker.track_assignment(0, Assignment::builder(1).name("Test 2").value(50.0).build())?;
    tracker.track_assignment(1, Assignment::builder(2).name("Test 1").value(50.0).build())?;
    tracker.track_assignment(1, Assignment::builder(3).name("Test 2").value(50.0).build())?;
    tracker.track_assignment(2, Assignment::builder(4).name("Test 1").value(50.0).build())?;
    tracker.track_assignment(2, Assignment::builder(5).name("Test 2").value(50.0).build())?;
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
