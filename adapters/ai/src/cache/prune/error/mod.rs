//! Prune error exports — xuất lỗi typed riêng cho AI cache pruning.

mod error;

pub(super) use error::{PruneError, PruneOutcome};

#[cfg(test)]
#[path = "test/error.rs"]
mod tests;
