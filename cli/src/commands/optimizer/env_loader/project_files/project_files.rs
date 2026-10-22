//! Safe optimizer-directory validation and project text reading.
//! (Kiểm tra thư mục optimizer an toàn và đọc text của project.)

use anyhow::Result;
use std::fs;
use std::path::Path;

/// Validate the optimizer directory without following a symlink.
/// Kiểm tra thư mục optimizer mà không theo symlink.
pub(in crate::commands::optimizer::env_loader) fn validate_optimizer_directory(
    path: &Path,
) -> Result<bool> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_dir() => {
            Err(crate::commands::optimizer::env_loader::error::directory_invalid(path))
        }
        Ok(_) => Ok(true),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(error) => Err(
            crate::commands::optimizer::env_loader::error::config_read_failed(path).context(error),
        ),
    }
}

/// Read a regular optimizer file through the shared no-follow project reader.
/// Đọc file optimizer thường qua bộ đọc project dùng chung có no-follow.
pub(in crate::commands::optimizer::env_loader) fn read_optimizer_text(
    path: &Path,
) -> Result<String> {
    match mgc_config::project::read_regular_project_text(path, "optimizer configuration") {
        Ok(Some(contents)) => Ok(contents),
        Ok(None) => Err(crate::commands::optimizer::env_loader::error::config_missing(path)),
        Err(error) => Err(
            crate::commands::optimizer::env_loader::error::config_read_failed(path).context(error),
        ),
    }
}
