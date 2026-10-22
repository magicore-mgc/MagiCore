//! Read an explicit core hint from a Node package manifest.
//! Đọc core hint tường minh trong manifest Node.

use anyhow::Result;
use mgc_config::project::read_regular_project_text;
use serde_json::Value;
use std::path::Path;

pub(in crate::dispatch) fn declared_core(root: &Path) -> Result<Option<String>> {
    let path = root.join("package.json");
    let Some(content) = read_regular_project_text(&path, "package manifest")? else {
        return Ok(None);
    };
    let manifest: Value = serde_json::from_str(&content)?;
    Ok(manifest
        .get("magicore")
        .and_then(|magicore| magicore.get("core"))
        .and_then(Value::as_str)
        .map(str::to_owned))
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
