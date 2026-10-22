// Plugin owner wiring and public export.
// Nối owner Plugin và export công khai.
#[path = "adapter_bridge/mod.rs"]
mod adapter_bridge;
#[path = "plugin.rs"]
mod implementation;
pub use implementation::Plugin;
