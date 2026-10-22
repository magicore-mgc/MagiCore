//! Read explicit core identity from pyproject.toml.
//! Đọc identity core tường minh từ pyproject.toml.
use std::path::Path;

use super::super::super::error::Result;
use super::super::super::signatures::signature_file_exists;
use crate::project::project::{ProjectConfig, file_access::read_regular_project_text};

/// Return the canonical pyproject.toml core declaration when present.
/// Trả về khai báo core pyproject.toml đã chuẩn hóa nếu có.
pub(in crate::project::project::core_detection::manifest_declarations) fn explicit_core(
    project_root: &Path,
) -> Result<Option<String>> {
    let path = project_root.join("pyproject.toml");
    if !signature_file_exists(&path)? {
        return Ok(None);
    }
    let Some(content) = read_regular_project_text(&path, "Python manifest")? else {
        return Ok(None);
    };
    let value: toml::Value = toml::from_str(&content).map_err(anyhow::Error::from)?;
    let core = value
        .get("tool")
        .and_then(|value| value.get("magicore"))
        .and_then(|value| value.get("core"))
        .and_then(toml::Value::as_str)
        .map(ProjectConfig::canonical_core);
    Ok(core)
}
