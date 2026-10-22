//! Ordered package-resolution phase wiring.
//! Wiring phase resolve package theo thứ tự.

#[path = "package_resolution.rs"]
mod implementation;

pub(crate) use implementation::resolve_requested_packages;
