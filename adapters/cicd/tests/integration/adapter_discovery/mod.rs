//! Adapter Discovery owner wiring — wiring cho owner adapter discovery.

#[path = "adapter_discovery.rs"]
mod adapter_discovery;
pub use adapter_discovery::{
    adapter_for_returns_none_without_markers, adapter_for_returns_some_with_wrangler,
    adapter_provider_method_returns_correct_str, can_handle_returns_true_for_known_marker,
};
