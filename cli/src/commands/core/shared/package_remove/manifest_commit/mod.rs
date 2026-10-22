//! Remove manifest-commit wiring.
//! Wiring commit manifest khi remove.

#[path = "manifest_commit.rs"]
mod implementation;

pub(crate) use implementation::commit_removed_manifest;
