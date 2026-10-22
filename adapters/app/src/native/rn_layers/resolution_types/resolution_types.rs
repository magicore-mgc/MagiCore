//! Public result containing the merged graph and non-JavaScript lock entries.
//! Kết quả công khai gồm graph gộp và entry lock ngoài JavaScript.

use mgc_types::ResolvedGraph;

/// A layered React Native resolution: merged graph plus non-JS lock entries.
/// Kết quả resolve React Native: graph gộp và entry lock các tier ngoài JS.
pub struct RNResolution {
    pub graph: ResolvedGraph,
    pub lock_packages: Vec<mgc_lockfile::Package>,
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
