//! store stats owner wiring.
//! Wiring owner store stats.

#[path = "stats.rs"]
mod implementation;

pub use implementation::{StoreStats, collect, print};
