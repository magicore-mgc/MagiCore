//! Prune type exports — xuất kiểu strategy và kết quả prune ổn định.

mod types;

pub use types::{PruneResult, PruneStrategy};

#[cfg(test)]
#[path = "test/types.rs"]
mod tests;
