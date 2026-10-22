//! Pool owner wiring.
//! Wiring owner pool.

#[path = "pool.rs"]
mod implementation;

pub use implementation::{default_hardlink_threads, hardlink_pool, hardlink_thread_count};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
