//! Native adapter validation wiring — wiring xác minh adapter native.
mod native_adapter_gate;
pub use native_adapter_gate::gate_native_adapter;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
