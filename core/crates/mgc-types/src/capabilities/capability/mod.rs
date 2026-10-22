//! Capability wire contract module wiring.
//! Wiring module hợp đồng wire capability.

mod capability;
pub use self::capability::Capability;
#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
