//! Fail-closed CLI boundary validation for `create-<core>`.
//! Validation fail-closed tại boundary CLI cho `create-<core>`.

use anyhow::Result;
use std::path::Path;

/// Fail-closed CLI input gate — a project name from the user must be a
/// single normal path segment; separators, "..", absolute paths, drive
/// prefixes, and home prefixes are rejected.
/// Cổng chặn tại CLI boundary — tên project từ user phải là 1 path segment
/// đơn lẻ; chặn separator, "..", absolute path, drive prefix, "~".
pub fn validate_project_name(project_name: &str) -> Result<()> {
    let valid = !project_name.is_empty()
        && !project_name.contains('/')
        && !project_name.contains('\\')
        && !project_name.contains("..")
        && !project_name.starts_with('~')
        && !project_name.contains(':')
        && Path::new(project_name)
            .components()
            .all(|c| matches!(c, std::path::Component::Normal(_)));
    if valid {
        Ok(())
    } else {
        Err(crate::error::invalid_project_name(project_name))
    }
}
