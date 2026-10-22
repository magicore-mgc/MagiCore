//! Dev Script output owner wiring.
//! Wiring owner ghi output dev script.

#[path = "dev_script.rs"]
mod implementation;
pub(super) use implementation::write;
