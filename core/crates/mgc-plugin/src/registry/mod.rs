// Registry owner wiring and public item exports.
// Nối owner registry và export các mục công khai.
#[path = "registry.rs"]
mod implementation;
pub use implementation::{PluginRegistry, global, register};
