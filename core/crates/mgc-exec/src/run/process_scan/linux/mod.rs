//! Linux process scan facade — export Linux process snapshots.
//! Facade quét process Linux — export snapshot process Linux.

#[path = "linux.rs"]
mod implementation;
pub(super) use implementation::{linux_process_snapshots, linux_process_stat_snapshot};
