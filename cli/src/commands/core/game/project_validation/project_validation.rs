//! Validate that the selected project is a Bevy package with a regular Cargo manifest.
//! Xác nhận project được chọn là Bevy và Cargo manifest là file thường.
use std::path::Path;

use anyhow::Result;

use super::super::error::GameOptimizerError;

/// Check optimizer ownership before any caller performs a side effect.
/// Kiểm tra quyền sở hữu optimizer trước khi caller tạo side effect.
#[cfg(feature = "game")]
pub(crate) fn validate_game_optimizer_project(root: &Path) -> Result<()> {
    require_bevy_optimizer_project(root).map(|_| ())
}
/// Resolve the Game adapter and require a regular native manifest before mutation.
/// Tìm Game adapter và yêu cầu native manifest dạng file thường trước mutation.
#[cfg(feature = "game")]
pub(in crate::commands::core::game) fn require_bevy_optimizer_project(
    root: &Path,
) -> Result<mgc_game_adapter::GameAdapter> {
    let adapter = mgc_game_adapter::adapter_for(root)
        .ok_or_else(|| GameOptimizerError::project_not_detected(root))?;
    if adapter.engine() != "bevy" {
        return Err(GameOptimizerError::unsupported_engine(adapter.engine()).into());
    }
    let manifest = root.join("Cargo.toml");
    let metadata = std::fs::symlink_metadata(&manifest).map_err(|error| {
        if error.kind() == std::io::ErrorKind::NotFound {
            GameOptimizerError::native_manifest_missing()
        } else {
            GameOptimizerError::native_manifest_inspect(error)
        }
    })?;
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        return Err(GameOptimizerError::native_manifest_not_regular().into());
    }
    Ok(adapter)
}
#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
