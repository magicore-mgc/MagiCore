//! Ordered update-resolution wiring.
//! Wiring resolve cập nhật theo thứ tự.

#[path = "package_resolution.rs"]
mod implementation;

pub(crate) use implementation::resolve_update_targets;

#[cfg(test)]
pub(crate) use implementation::UpdateResolution;
