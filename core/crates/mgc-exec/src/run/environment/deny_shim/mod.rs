//! Deny-shim facade for platform-specific package-manager blockers.
//! Facade deny-shim cho blocker package manager theo nền tảng.

#[path = "deny_shim.rs"]
mod implementation;

#[cfg(unix)]
pub(in crate::run) use implementation::NPM_VERSION_PROBE_SHIM_CONTENT;
pub(in crate::run) use implementation::write_blocker;
