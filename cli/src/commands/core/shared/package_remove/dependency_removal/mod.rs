//! Dependency-removal phase wiring.
//! Wiring phase gỡ dependency.

#[path = "dependency_removal.rs"]
mod implementation;

pub(crate) use implementation::remove_requested_dependencies;
