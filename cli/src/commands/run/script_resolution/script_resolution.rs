//! Read script entries and apply source precedence.
//! Đọc mục script và áp dụng thứ tự ưu tiên nguồn.

use anyhow::Result;
use std::path::Path;

pub(crate) fn resolve_mgc_toml_script(path: &Path, script: &str) -> Result<Option<String>> {
    let content = std::fs::read_to_string(path)?;
    let toml: toml::Value = toml::from_str(&content)?;
    Ok(toml
        .get("scripts")
        .and_then(|s| s.get(script))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string()))
}

pub(crate) fn resolve_package_json_script(path: &Path, script: &str) -> Result<Option<String>> {
    let content = std::fs::read_to_string(path)?;
    let manifest: serde_json::Value = serde_json::from_str(&content)?;
    Ok(manifest
        .get("scripts")
        .and_then(|s| s.get(script))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string()))
}

/// Resolve a task script with source precedence (Spec rev4 §16.2, decided):
/// `mgc.toml [scripts]` wins; the ecosystem manifest is the fallback.
/// Returns the command and whether it came from `mgc.toml`.
/// (Resolve task script kèm ưu tiên nguồn (Spec rev4 §16.2, đã chốt):
/// `mgc.toml [scripts]` thắng; manifest ecosystem là fallback. Trả lệnh và
/// cờ cho biết có từ `mgc.toml` không.)
pub(crate) fn resolve_task_script(
    project_root: &Path,
    script: &str,
) -> Result<Option<(String, bool)>> {
    let mgc_toml_path = project_root.join("mgc.toml");
    if mgc_toml_path.exists()
        && let Some(cmd) = resolve_mgc_toml_script(&mgc_toml_path, script)?
    {
        return Ok(Some((cmd, true)));
    }
    let package_json_path = project_root.join("package.json");
    if package_json_path.exists()
        && let Some(cmd) = resolve_package_json_script(&package_json_path, script)?
    {
        return Ok(Some((cmd, false)));
    }
    Ok(None)
}
