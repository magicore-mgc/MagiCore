//! Generate a process-local identity propagated to monitored descendants.
//! Sinh identity trong process để truyền cho descendant được monitor.

use std::sync::atomic::{AtomicU64, Ordering};

pub(in crate::run::implementation) const PROCESS_TREE_TRACKING_ENV: &str =
    "MGC_EXEC_PROCESS_TREE_ID";

static PROCESS_TREE_TRACKING_SEQUENCE: AtomicU64 = AtomicU64::new(0);

pub(in crate::run::implementation) fn process_tree_tracking_token() -> String {
    let sequence = PROCESS_TREE_TRACKING_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |duration| duration.as_nanos());
    format!("{:x}-{sequence:x}-{timestamp:x}", std::process::id())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
