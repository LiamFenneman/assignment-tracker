pub mod assignment;
pub mod course;

type Result<T> = std::result::Result<T, TrackerError>;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum TrackerError {
    #[error("`{0}` is not within a percentage range (0..=100)")]
    OutOfPercentageRange(u32),
    #[error("unknown tracker error")]
    Unknown,
}
