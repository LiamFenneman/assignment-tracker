//! # Tracker Core
//! Core utility to keep track of classes and assignments.
#![warn(clippy::pedantic)]
#![feature(let_else)]
#![feature(is_some_with)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate anyhow;

pub mod assignment;
pub mod class;
pub mod mark;
pub mod tracker;

pub use tracker::Tracker;

pub mod prelude {
    pub use crate::mark::Mark;

    pub use crate::class::Class;
    pub use crate::class::Classlike;
    pub use crate::class::Code;

    pub use crate::assignment::Assignment;
    pub use crate::assignment::Assignmentlike;

    pub use crate::tracker::Tracker;
    pub use crate::tracker::Trackerlike;
}
