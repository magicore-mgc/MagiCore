//! Refuse missing, linked or special project-config paths before mutation.
//! Từ chối path cấu hình thiếu, liên kết hoặc đặc biệt trước khi sửa.
use std::path::Path;

use super::super::error::{FileAccessError, Result};

/// Preserve the existing helper boundary used by project identity owners.
/// Giữ biên helper hiện tại cho các owner project identity.
pub(in crate::project::project) fn ensure_regular_project_config(
    path: &Path,
) -> anyhow::Result<Option<std::fs::Metadata>> {
    ensure_regular_project_config_typed(path).map_err(FileAccessError::into_anyhow)
}

/// Return metadata only for an existing regular config file.
/// Chỉ trả metadata nếu config hiện hữu là file thường.
pub(in crate::project::project::file_access) fn ensure_regular_project_config_typed(
    path: &Path,
) -> Result<Option<std::fs::Metadata>> {
    let metadata = match std::fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(error.into()),
    };
    if metadata.file_type().is_symlink() {
        return Err(FileAccessError::Symlink {
            label: "project config".to_owned(),
            path: path.to_path_buf(),
        });
    }
    if !metadata.is_file() {
        return Err(FileAccessError::NotRegular {
            label: "project config".to_owned(),
            path: path.to_path_buf(),
        });
    }
    Ok(Some(metadata))
}
