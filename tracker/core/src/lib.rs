#![warn(clippy::pedantic)]
#![feature(let_else)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate anyhow;

/// Maximum length of the name of an [assignment](Assignment)/[class](Class) in bytes.
pub const MAX_NAME_LEN: usize = 32;

mod assignment;
pub use assignment::Assignment;

mod class;
pub use class::Class;

mod tracker;
pub use tracker::Tracker;

/// Shorthand to call [`bail!`] and [`error!`] macros with the same message.
#[macro_export]
macro_rules! err {
    ($msg:expr) => {
        error!($msg);
        bail!($msg);
    };
}
