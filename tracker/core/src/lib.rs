#![warn(clippy::pedantic)]
#![feature(let_else)]

/// Maximum length of the name of an [assignment](Assignment)/[class](Class) in bytes.
pub const MAX_NAME_LEN: usize = 32;

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
