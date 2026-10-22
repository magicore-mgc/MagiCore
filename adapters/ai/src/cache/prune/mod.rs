//! AI cache prune exports — khai báo API prune và owner thực thi gần nhất.

mod candidate_discovery;
mod error;
mod policy;
mod prune;
mod removal;
mod types;

pub use prune::{prune_cache, remove_model};
pub use types::{PruneResult, PruneStrategy};

#[cfg(test)]
#[path = "test/prune.rs"]
mod tests;
