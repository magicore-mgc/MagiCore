//! React Native layered resolution/install public facade and owner wiring.
//! Facade công khai resolve/install phân tầng React Native và nối các owner.

mod graph_subset;
pub(super) mod install_layers;
pub(super) mod install_maven;
pub(super) mod lock_markers;
pub(super) mod resolution_types;
pub(super) mod resolve_android;
pub(super) mod resolve_ios;
pub(super) mod resolve_javascript;
pub(super) mod resolve_layers;
pub(super) mod resolve_pod;

pub use install_layers::install_rn_layers;
pub use resolution_types::RNResolution;
pub use resolve_layers::resolve_rn_layers;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
