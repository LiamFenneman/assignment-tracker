pub mod assignment;
pub use assignment::Assignment;

mod class;
pub use class::Class;

#[macro_export]
macro_rules! err {
    ($msg:expr) => {
        error!($msg);
        bail!($msg);
    };
}
