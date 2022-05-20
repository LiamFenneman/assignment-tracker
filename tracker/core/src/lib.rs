#![feature(let_else)]

pub mod assignment;
pub use assignment::Assignment;

mod class;
pub use class::Class;

mod tracker;
pub use tracker::Tracker;

#[macro_export]
macro_rules! err {
    ($msg:expr) => {
        error!($msg);
        bail!($msg);
    };
}
