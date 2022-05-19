use anyhow::Result;
use pretty_env_logger;
use tracker_core::*;

fn main() -> Result<()> {
    pretty_env_logger::init();
    let mut a = Assignment::builder(0) //
        .name("Test")
        .value(50.0)
        .build();

    a.set_mark(100.0)?;
    a.remove_mark();
    a.remove_mark();
    let _ = a.set_mark(-100.0);

    Ok(())
}
