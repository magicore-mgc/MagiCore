// Adapter bridge owner wiring and visibility boundary.
// Nối owner adapter bridge và giới hạn visibility.
#[path = "adapter_bridge.rs"]
mod implementation;
pub(super) use implementation::{
    CacheUnsupported, FetcherBridge, LinkerBridge, ResolverBridge, TemplateUnsupported,
};
