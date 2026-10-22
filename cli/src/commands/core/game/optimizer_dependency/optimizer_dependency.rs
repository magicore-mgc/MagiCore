//! Add the local optimizer path without overwriting conflicting user data.
//! Thêm path optimizer local mà không ghi đè dữ liệu người dùng bị xung đột.
use std::path::Path;

use super::super::error::GameOptimizerError;
use crate::commands::core::shared::{atomic_write_file_checked, refuse_project_link};
use anyhow::Result;

/// Add the local optimizer path without overwriting a conflicting user value.
/// Thêm optimizer path local, không ghi đè giá trị khác của người dùng.
#[cfg(feature = "game")]
pub(in crate::commands::core::game) async fn game_edit_optimizer_dep(root: &Path) -> Result<()> {
    let manifest = root.join("Cargo.toml");
    let metadata = std::fs::symlink_metadata(&manifest).map_err(|error| {
        if error.kind() == std::io::ErrorKind::NotFound {
            GameOptimizerError::hook_manifest_missing()
        } else {
            GameOptimizerError::hook_manifest_inspect(error)
        }
    })?;
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        return Err(GameOptimizerError::hook_manifest_not_regular().into());
    }
    refuse_project_link(&manifest)?;
    let content = std::fs::read_to_string(&manifest)?;
    let mut manifest_value: toml::Value = toml::from_str(&content)?;
    let dependencies = manifest_value["dependencies"]
        .as_table_mut()
        .ok_or_else(GameOptimizerError::dependency_section_missing)?;
    match dependencies.get("mgc-optimizer") {
        Some(existing)
            if existing.get("path").and_then(|path| path.as_str()) == Some("./optimizer") =>
        {
            return Ok(());
        }
        Some(existing) => {
            let path = existing
                .get("path")
                .and_then(|path| path.as_str())
                .unwrap_or("");
            return Err(GameOptimizerError::conflicting_dependency(path).into());
        }
        None => {}
    }
    dependencies.insert(
        "mgc-optimizer".to_string(),
        toml::Value::Table(toml::map::Map::from_iter([(
            "path".to_string(),
            toml::Value::String("./optimizer".to_string()),
        )])),
    );
    atomic_write_file_checked(
        root,
        "Cargo.toml",
        toml::to_string_pretty(&manifest_value)?.as_bytes(),
        false,
    )?;

    // Verify the persisted value before reporting success.
    // Chỉ báo thành công sau khi xác minh giá trị đã ghi.
    let reread: toml::Value = toml::from_str(&std::fs::read_to_string(&manifest)?)?;
    let confirmed = reread
        .get("dependencies")
        .and_then(|dependencies| dependencies.get("mgc-optimizer"))
        .and_then(|dependency| dependency.get("path"))
        .and_then(|path| path.as_str())
        == Some("./optimizer");
    if !confirmed {
        return Err(GameOptimizerError::manifest_round_trip_failed().into());
    }
    Ok(())
}
#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
