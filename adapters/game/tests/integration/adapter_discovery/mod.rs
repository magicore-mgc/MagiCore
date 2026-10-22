//! Adapter Discovery owner wiring — wiring cho owner adapter discovery.

#[path = "adapter_discovery.rs"]
mod adapter_discovery;
pub use adapter_discovery::{
    adapter_for_returns_none_for_plain_dir, adapter_for_returns_some_for_godot,
};
