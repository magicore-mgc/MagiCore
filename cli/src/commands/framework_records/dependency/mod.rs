//! Dependency projection wiring — nối projection dependency.
#[path = "dependency.rs"]
mod implementation;

pub(super) use self::implementation::{dependency_manifest_format, dependency_ownership_json};
