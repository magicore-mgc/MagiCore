//! Safely read `mgc.toml` and extract its ecosystem value.
//! Đọc an toàn `mgc.toml` và lấy giá trị ecosystem.
use std::path::Path;

use super::super::ecosystem_from_config::ecosystem_from_config_typed;
use super::super::error::{FileAccessError, Result as FileAccessResult};
use super::super::regular_project_text::read_regular_project_text_typed;

/// Preserve the existing helper signature used by core detection/identity.
/// Giữ chữ ký helper đang dùng bởi core detection/identity.
pub(in crate::project::project) fn read_to_string_ecosystem(
    project_root: &Path,
) -> Result<Option<String>, anyhow::Error> {
    read_to_string_ecosystem_typed(project_root).map_err(FileAccessError::into_anyhow)
}

fn read_to_string_ecosystem_typed(project_root: &Path) -> FileAccessResult<Option<String>> {
    let Some(content) =
        read_regular_project_text_typed(&project_root.join("mgc.toml"), "project config")?
    else {
        return Ok(None);
    };
    ecosystem_from_config_typed(Some(&content))
}
