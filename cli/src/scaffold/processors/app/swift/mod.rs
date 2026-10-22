//! Swift Package owner wiring.
//! Wiring owner Swift Package.

#[path = "swift.rs"]
mod implementation;
pub(super) use implementation::write;
