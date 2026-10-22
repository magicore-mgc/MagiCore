//! Simulator provider capability module wiring.
//! Wiring module capability simulator provider.

mod simulator_provider;
pub use self::simulator_provider::SimulatorProvider;
#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
