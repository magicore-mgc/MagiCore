//! Adapter Discovery owner wiring — wiring cho owner adapter discovery.

#[path = "adapter_discovery.rs"]
mod adapter_discovery;
pub use adapter_discovery::{
    adapter_cloud_type_method_returns_correct_str, adapter_for_returns_none_without_any_marker,
    adapter_for_returns_some_for_terraform,
};
