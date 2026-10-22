//! Read explicit core identity from package.json.
//! Đọc identity core tường minh trong package.json.
use std::path::Path;

use super::super::super::error::Result;
use super::super::super::signatures::signature_file_exists;
use crate::project::project::{ProjectConfig, file_access::read_regular_project_text};

/// Return the canonical package.json core declaration when present.
/// Trả về khai báo core package.json đã chuẩn hóa nếu có.
pub(in crate::project::project::core_detection::manifest_declarations) fn explicit_core(
    project_root: &Path,
) -> Result<Option<String>> {
    let path = project_root.join("package.json");
    if !signature_file_exists(&path)? {
        return Ok(None);
    }
    let Some(content) = read_regular_project_text(&path, "package manifest")? else {
        return Ok(None);
    };
    let value: serde_json::Value = serde_json::from_str(&content).map_err(anyhow::Error::from)?;
    let core = value
        .get("magicore")
        .and_then(|value| value.get("core"))
        .and_then(serde_json::Value::as_str)
        .map(ProjectConfig::canonical_core);
    Ok(core)
}
