//! Native npm probe authentication facade.
//! Facade xác thực npm probe native.

#[path = "error.rs"]
mod error;
#[path = "npm_probe.rs"]
mod implementation;

#[cfg(all(windows, test))]
pub(in crate::run::implementation) use implementation::is_shadow_windows_npm_version_probe;
#[cfg(any(windows, test))]
pub(in crate::run::implementation) use implementation::windows_npm_probe_authentication;
