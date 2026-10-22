//! Update manifest-commit wiring.
//! Wiring commit manifest khi update.

#[path = "manifest_commit.rs"]
mod implementation;

pub(crate) use implementation::commit_update_manifest;
