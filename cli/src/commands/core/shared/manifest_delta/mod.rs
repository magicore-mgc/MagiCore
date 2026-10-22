//! Shared pinned-manifest and resolved-graph delta operations.
//! عملیات delta مشترک برای manifest pin‌شده و گراف resolveشده.

#[path = "error.rs"]
mod error;
#[path = "manifest_delta.rs"]
mod implementation;

pub(crate) use implementation::{AddedPackage, build_delta_manifest, merge_graphs};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
