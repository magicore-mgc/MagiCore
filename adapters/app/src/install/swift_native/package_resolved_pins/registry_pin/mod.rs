//! Wires the private registry_pin owner modules.
//! Nối các module private thuộc owner registry_pin.

mod registry_pin;
pub(in crate::install::swift_native) use registry_pin::build_registry_pin;
