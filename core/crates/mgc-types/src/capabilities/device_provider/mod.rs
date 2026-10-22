//! Device provider capability module wiring.
//! Wiring module capability device provider.

mod device_provider;
pub use self::device_provider::DeviceProvider;
#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
