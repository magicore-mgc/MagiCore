//! Store measurement owner wiring.
//! Wiring owner đo lường store.

#[path = "stats/mod.rs"]
mod stats;

pub use stats::{StoreStats, collect, print};
