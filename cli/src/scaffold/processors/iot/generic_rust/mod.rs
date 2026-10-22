//! Generic Rust writer owner wiring.
//! Wiring owner writer rust tổng quát.

#[path = "generic_rust.rs"]
mod implementation;
pub(super) use implementation::write;
