//! Generic cache-file pruning owner wiring.
//! File này chỉ nối module files và export API nội bộ.

mod files;

pub(in crate::commands::cache) use files::{prune_files_under, prune_unlinked_files_under};
