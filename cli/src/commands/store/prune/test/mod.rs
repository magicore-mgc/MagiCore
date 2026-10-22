//! Nearest tests for store pruning side effects and lock behavior.
//! Test gần owner cho side effect và lock của store prune.

#[path = "prune.rs"]
mod prune;
#[path = "report.rs"]
mod report;
#[cfg(any(unix, windows))]
#[path = "symlink_support.rs"]
mod symlink_support;
