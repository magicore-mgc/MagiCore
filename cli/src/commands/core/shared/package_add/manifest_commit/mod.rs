//! Journal-aware manifest commit phase wiring.
//! Wiring phase commit manifest có journal.

#[path = "manifest_commit.rs"]
mod implementation;

pub(crate) use implementation::commit_manifest_changes;
