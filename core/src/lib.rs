//! # Tracker Core
//! Core library for tracking courses and assignments.

#![warn(clippy::pedantic)]

pub mod assignment;
pub mod course;

// Hide the private organization of the mark module from the docs.
mod marks {
    pub mod grade;
    mod mark;
    pub mod out_of;
    pub mod percent;
    pub use grade::Grade;
    pub use mark::Mark;
    pub use out_of::OutOf;
    pub use percent::Percent;
}
pub mod mark {
    pub use crate::marks::*;
}
