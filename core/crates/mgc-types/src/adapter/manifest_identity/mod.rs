//! Manifest Identity owner exports — xuất định danh manifest.

mod manifest_identity;
pub use manifest_identity::ManifestIdentity;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
