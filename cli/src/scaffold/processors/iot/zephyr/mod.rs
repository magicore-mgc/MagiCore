//! Zephyr writer owner wiring.
//! Wiring owner writer zephyr.

#[path = "zephyr.rs"]
mod implementation;
pub(super) use implementation::write;
