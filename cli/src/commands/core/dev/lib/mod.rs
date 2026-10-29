//! Lib dev owner wiring — wiring owner dev Lib.

#[path = "error/mod.rs"]
pub(super) mod error;
#[path = "lib.rs"]
mod implementation;

pub use implementation::dev;
