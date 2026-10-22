//! Read original manifest bytes and restore them after failed publication.
//! Đọc byte manifest gốc và khôi phục khi publish thất bại.
use std::path::Path;

use super::super::error::GameOptimizerError;
use crate::commands::core::shared::atomic_write_file_checked;
use anyhow::Result;

/// Read the manifest snapshot required to restore the exact pre-operation bytes.
/// Đọc snapshot manifest để khôi phục chính xác byte trước thao tác.
#[cfg(feature = "game")]
pub(in crate::commands::core::game) fn read_manifest_snapshot(manifest: &Path) -> Result<Vec<u8>> {
    std::fs::read(manifest)
        .map_err(|source| anyhow::Error::new(GameOptimizerError::manifest_snapshot_failed(source)))
}
#[cfg(feature = "game")]
pub(in crate::commands::core::game) fn restore_manifest_snapshot(
    root: &Path,
    snapshot: &[u8],
    operation: &str,
    original_error: anyhow::Error,
) -> anyhow::Error {
    match atomic_write_file_checked(root, "Cargo.toml", snapshot, false) {
        Ok(()) => original_error,
        Err(source) => {
            GameOptimizerError::manifest_rollback_failed(operation, &original_error, source).into()
        }
    }
}
#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
