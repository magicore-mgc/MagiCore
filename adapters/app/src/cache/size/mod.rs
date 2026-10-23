//! App cache size owner.
//! Owner dung lượng cache App.

mod size;

pub use size::cache_size;
#[cfg(test)]
pub(crate) use size::dir_size;
