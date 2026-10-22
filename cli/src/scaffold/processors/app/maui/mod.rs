//! MAUI owner wiring.
//! Wiring owner MAUI.

#[path = "maui.rs"]
mod implementation;
pub(super) use implementation::write;
