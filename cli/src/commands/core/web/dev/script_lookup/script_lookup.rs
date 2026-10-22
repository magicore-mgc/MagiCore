//! Reads declared package scripts and applies the supported dev fallback.
//! Đọc script package đã khai báo và áp dụng fallback dev được hỗ trợ.

use anyhow::Result;
use std::path::Path;

use super::super::types::DevPackageJson;

pub(in crate::commands::core::web::implementation) fn read_script(
    project_root: &Path,
    script_name: &str,
) -> Result<Option<String>> {
    let package_json_path = project_root.join("package.json");
    if !package_json_path.exists() {
        return Ok(None);
    }

    let parsed: DevPackageJson =
        serde_json::from_str(&std::fs::read_to_string(package_json_path)?)?;
    let scripts = match parsed.scripts {
        Some(scripts) => scripts,
        None => return Ok(None),
    };

    if let Some(script) = scripts.get(script_name).cloned() {
        return Ok(Some(script));
    }

    if script_name == "dev" {
        return Ok(scripts.get("start").cloned());
    }

    Ok(None)
}

pub(in crate::commands::core::web::implementation) fn read_dev_script(
    project_root: &Path,
) -> Result<Option<String>> {
    read_script(project_root, "dev")
}
