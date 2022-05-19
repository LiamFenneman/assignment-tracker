//! # Library
//!
//! This crate provides the all functionality which is required to track and manage assignments.
//!
//! # Example
//! The following is all you need to get started working with the library.
//! ```
//! # use std::rc::Rc;
//! # use tracker_core::{Assignment, Tracker};
//! // Create the tracker
//! let mut tracker = Tracker::new();
//!
//! // Get a class code
//! let code = tracker.get_code("EXPL101")?;
//!
//! // Create assignments
//! let mut assign_1 = Assignment::new("Example 1", 25.0, Rc::clone(&code))?;
//! let mut assign_2 = Assignment::new("Example 2", 25.0, Rc::clone(&code))?;
//! let assign_3 = Assignment::new("Example 3", 50.0, Rc::clone(&code))?;
//!
//! // Set marks for assignments
//! assign_1.set_mark(75.0)?;
//! assign_2.set_mark(50.0)?;
//!
//! // Add the assignments to the tracker
//! tracker.track(assign_1)?;
//! tracker.track_many(vec![assign_2, assign_3])?;
//!
//! // Check that the 3 assignments were added
//! assert_eq!(3, tracker.get_all().len());
//! # Ok::<(), &'static str>(())
//! ```

#[macro_use]
extern crate lazy_static;

pub mod assignment;
pub use assignment::Assignment;

mod class_code;
pub use class_code::ClassCode;

mod tracker;
pub use tracker::Tracker;
