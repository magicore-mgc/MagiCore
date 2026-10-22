//! Lifecycle runner capability module wiring.
//! Wiring module capability lifecycle runner.

mod lifecycle_runner;
pub use self::lifecycle_runner::LifecycleRunner;
#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
