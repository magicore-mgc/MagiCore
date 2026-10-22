//! Go module and compiler policy facade.
//! Facade policy module và compiler Go.

#[path = "go.rs"]
mod implementation;
pub(in crate::run) use implementation::denial_reason;
#[cfg(test)]
pub(in crate::run) use implementation::is_go_toolchain_version_probe;
