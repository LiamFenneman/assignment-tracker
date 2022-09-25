//! # Tracker Core
//! Core library for tracking courses and assignments.

#![warn(clippy::pedantic)]

#[macro_use]
extern crate log;

pub mod assignment;
pub mod course;

mod util {
    pub mod percent;
}
pub use util::percent;