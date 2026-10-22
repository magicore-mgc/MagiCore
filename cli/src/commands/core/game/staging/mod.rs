//! Optimizer staging ownership and exports.
//! Quản lý staging của optimizer và export tối thiểu.

#[path = "staging.rs"]
mod implementation;

#[cfg(feature = "game")]
pub(super) use implementation::{
    cleanup_after_failure, prepare_staging_directory, remove_staging_directory,
};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
