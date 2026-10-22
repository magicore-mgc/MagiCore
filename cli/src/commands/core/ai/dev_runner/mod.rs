//! AI development-runtime owner wiring.
//! Wiring owner runtime dev AI.

mod dev_runner;

#[cfg(feature = "ai")]
pub use dev_runner::ai_dev;
