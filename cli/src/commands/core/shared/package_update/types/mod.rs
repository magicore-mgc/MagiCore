//! Update-target type wiring.
//! Wiring type target update.

#[path = "types.rs"]
mod implementation;

pub(in crate::commands::core::shared) use implementation::UpdateTarget;
