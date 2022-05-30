//! # Tracker Core
//! Core library to keep track of classes and assignments.
//!
//! # Examples
//! ### Tracker
//! The following shows the flow of creating a tracker that contains classes and assignments.
//! ```
//! # use anyhow::Result;
//! use tracker_core::prelude::*;
//! # fn main() -> Result<()> {
//!
//! // create a tracker (can use Class or Code)
//! let mut tracker = Tracker::<Class>::new("My Tracker");
//!
//! // create and add a class
//! let class = Class::with_name("CLASS 101", "My Class");
//! tracker.add_class(class)?;
//!
//! // create and add assignments
//! let a1 = Assignment::new(0, "Assignment 1", 50.0);
//! let a2 = Assignment::builder(1, "Assignment 2", 50.0)
//!     .mark(Mark::percent(30.0)?)
//!     .build();
//! tracker.add_assignment("CLASS 101", a1)?;
//! tracker.add_assignment("CLASS 101", a2)?;
//! # Ok(()) }
//! ```
//!
//! ### Mark
//! Use the [`Mark::percent()`](./mark/enum.Mark.html#method.percent), [`Mark::letter()`](./mark/enum.Mark.html#method.letter), [`Mark::out_of()`](./mark/enum.Mark.html#method.out_of) instead of enum variants to ensure the value is valid.
//! ```
//! # use anyhow::Result;
//! use tracker_core::prelude::*;
//! # fn main() -> Result<()> {
//!
//! let mark = Mark::percent(75.0)?;
//! let mark = Mark::percent(25.0)?;
//!
//! let mark = Mark::letter('A')?;
//! let mark = Mark::letter('D')?;
//!
//! let mark = Mark::out_of(7, 10)?;
//! let mark = Mark::out_of(22, 25)?;
//! # Ok(()) }
//! ```
//!
//! ### Assignment
//! ##### New Pattern
//! Use the following when you are not providing a mark or due date.
//! ```
//! # use anyhow::Result;
//! use chrono::NaiveDate;
//! use tracker_core::prelude::*;
//! # fn main() -> Result<()> {
//!
//! let mut assign = Assignment::new(0, "Assignment 1", 25.0);
//!
//! // can be updated later
//! assign.set_mark(Mark::percent(75.0)?)?;
//! assign.set_due_date(NaiveDate::from_ymd(2022, 12, 25).and_hms(12, 45, 30));
//! # Ok(()) }
//! ```
//!
//! ##### Builder Pattern
//! Use the builder pattern when you want to provide a mark and/or due date.
//! ```
//! # use anyhow::Result;
//! use chrono::NaiveDate;
//! use tracker_core::prelude::*;
//! # fn main() -> Result<()> {
//!
//! let assign = Assignment::builder(0, "Assignment 1", 25.0)
//!     .mark(Mark::letter('A')?)
//!     .due_date(NaiveDate::from_ymd(2022, 12, 25).and_hms(12, 45, 30))
//!     .build();
//! # Ok(()) }
//! ```
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
