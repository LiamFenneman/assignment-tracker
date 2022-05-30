#![warn(clippy::pedantic)]
#![feature(let_else)]
#![feature(is_some_with)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate anyhow;

mod mark;
pub use mark::InvalidMarkError;
pub use mark::Mark;

mod class;
pub use class::Class;
pub use class::Classlike;
pub use class::Code;

mod assignment;
pub use assignment::Assignment;
pub use assignment::Assignmentlike;

mod tracker;
pub use tracker::Tracker;
pub use tracker::Trackerlike;

#[macro_export]
macro_rules! err {
    ($msg:expr) => {
        error!($msg);
        bail!($msg);
    };
}
