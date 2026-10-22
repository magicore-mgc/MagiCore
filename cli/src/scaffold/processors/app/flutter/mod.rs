//! Flutter owner wiring.
//! Wiring owner Flutter.

#[path = "flutter.rs"]
mod implementation;
pub(super) use implementation::write;
