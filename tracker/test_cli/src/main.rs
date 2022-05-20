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

    a.set_mark(100.0)?;
    a.remove_mark();
    a.remove_mark();

    class.add_assignment(a)?;
    class.remove_assignment(0)?;

    Ok(())
}
