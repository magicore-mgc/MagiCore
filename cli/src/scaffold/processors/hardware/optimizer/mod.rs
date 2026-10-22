//! Hardware optimizer marker owner wiring.
//! Wiring owner marker hardware optimizer.

#[path = "optimizer.rs"]
mod implementation;
pub(super) use implementation::write;
