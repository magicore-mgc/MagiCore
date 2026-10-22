//! Refuse linked or non-directory optimizer destinations.
//! Từ chối đích optimizer là link hoặc không phải thư mục.
use std::path::Path;

use super::super::error::GameOptimizerError;
use crate::commands::core::shared::refuse_project_link;
use anyhow::Result;

/// Validate an existing optimizer path without following a project link.
/// Kiểm tra path optimizer hiện có mà không đi theo link của project.
#[cfg(feature = "game")]
pub(in crate::commands::core::game) fn validate_optimizer_destination(path: &Path) -> Result<bool> {
    refuse_project_link(path).map_err(|source| {
        anyhow::Error::new(GameOptimizerError::template_destination_rejected(
            path, source,
        ))
    })?;
    match std::fs::symlink_metadata(path) {
        Ok(metadata) if metadata.is_dir() && !metadata.file_type().is_symlink() => Ok(true),
        Ok(_) => Err(GameOptimizerError::template_destination_not_directory(path).into()),
        Err(source) if source.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(source) => {
            Err(GameOptimizerError::template_destination_inspect_failed(path, source).into())
        }
    }
}
#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
