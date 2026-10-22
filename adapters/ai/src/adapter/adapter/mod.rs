//! AI adapter capability owner wiring.
//! Wiring các owner capability của adapter AI.

mod adapter;
mod audit;
mod capability_defaults;
mod capability_manifest;
mod identity;
mod model_artifact;
mod native_python_capabilities;
mod package_adapter;

pub use adapter::{AiAdapter, adapter_for, uses_native_python_lane};
