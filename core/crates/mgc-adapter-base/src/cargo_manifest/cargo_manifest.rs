//! Stable public API for Cargo dependency-manifest operations.
//! API public ổn định cho thao tác dependency manifest Cargo.

use mgc_types::error::MgResult;
use mgc_types::{Ecosystem, Manifest};
use std::path::Path;

/// Parse the Cargo dependency subset supported by MagiCore's native resolver.
/// Parse tập dependency Cargo mà resolver native của MagiCore hỗ trợ.
pub fn parse_manifest(root: &Path, ecosystem: Ecosystem) -> MgResult<Manifest> {
    super::parse_manifest::parse_manifest(root, ecosystem)
}

/// Update only the supported root dependency tables in Cargo.toml.
/// Chỉ cập nhật các bảng dependency root được hỗ trợ trong Cargo.toml.
pub fn write_manifest(root: &Path, manifest: &Manifest) -> MgResult<()> {
    super::write_manifest::write_manifest(root, manifest)
}

#[cfg(test)]
#[path = "test/cargo_manifest.rs"]
mod tests;
