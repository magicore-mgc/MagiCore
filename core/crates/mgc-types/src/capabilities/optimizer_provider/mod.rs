//! Optimizer provider capability module wiring.
//! Wiring module capability optimizer provider.

mod optimizer_provider;
pub use self::optimizer_provider::OptimizerProvider;
#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
