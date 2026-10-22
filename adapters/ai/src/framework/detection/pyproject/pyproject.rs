//! Reader for the `[tool.magicore].framework` pyproject key.
//! Bộ đọc key `[tool.magicore].framework` của pyproject.

use std::path::Path;

use super::super::AiFramework;
use super::super::framework_from_str;

/// Return a recognized framework configured in the regular project pyproject.
/// Trả framework được nhận diện trong pyproject dạng regular file.
pub(in crate::framework) fn framework_from_pyproject(root: &Path) -> Option<AiFramework> {
    let Ok(Some(content)) = mgc_config::project::read_regular_project_text(
        &root.join("pyproject.toml"),
        "Python project manifest",
    ) else {
        return None;
    };
    let Ok(value) = toml::from_str::<toml::Value>(&content) else {
        return None;
    };
    let name = value
        .get("tool")
        .and_then(|tool| tool.get("magicore"))
        .and_then(|magicore| magicore.get("framework"))
        .and_then(toml::Value::as_str)?;
    framework_from_str(name)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
