//! Hardware benchmark marker owner wiring.
//! Wiring owner marker benchmark hardware.

#[path = "bench.rs"]
mod implementation;
pub(super) use implementation::write;
