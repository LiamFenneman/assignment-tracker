#[macro_use]
extern crate lazy_static;

mod assignment;
pub use assignment::Assignment;

mod class_code;
pub use class_code::{ClassCode, ClassCodes};

mod tracker;
pub use tracker::Tracker;
