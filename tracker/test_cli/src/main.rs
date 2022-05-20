use anyhow::Result;
use pretty_env_logger;
use tracker_core::*;

fn main() -> Result<()> {
    pretty_env_logger::init();

    let mut class = Class::new(0, "TEST101");
    let mut a = Assignment::builder(0) //
        .name("Test")
        .value(50.0)
        .build();

    a.set_mark(55.0)?;
    a.remove_mark();
    a.remove_mark();

    let _ = class.add_assignment(a);
    let _ = class.add_mark(0, 75.0);
    let _ = class.add_mark(1, 10.0);
    let _ = class.add_mark(0, -10.0);
    let _ = class.remove_assignment(0);

    Ok(())
}
