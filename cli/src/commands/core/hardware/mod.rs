//! Hardware core materialization owner.
//! Owner materialize của Hardware core.

#[path = "error.rs"]
mod error;
#[path = "hardware.rs"]
mod implementation;

pub use implementation::{BENCH_PKG, OPTIMIZER_PKG, materialize_template};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
