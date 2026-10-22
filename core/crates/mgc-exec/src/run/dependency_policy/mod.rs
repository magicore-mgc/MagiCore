//! Dependency-resolution policy facade.
//! Facade chính sách resolve dependency.

#[path = "error.rs"]
mod error;
#[path = "dependency_policy.rs"]
mod implementation;

#[cfg(test)]
pub(in crate::run) use implementation::is_go_toolchain_version_probe;
pub(in crate::run) use implementation::{
    dependency_resolution_deny_reason, reject_external_dependency_resolution,
};
