//! Reader for the `[ai].framework` key in project `mgc.toml`.
//! Bộ đọc key `[ai].framework` trong `mgc.toml` của project.

use std::path::Path;

use super::super::AiFramework;
use super::super::framework_from_str;

/// Return a recognized framework configured in the regular project config.
/// Trả framework được nhận diện trong project config dạng regular file.
pub(in crate::framework) fn framework_from_project_config(root: &Path) -> Option<AiFramework> {
    let Ok(Some(content)) =
        mgc_config::project::read_regular_project_text(&root.join("mgc.toml"), "project config")
    else {
        return None;
    };
    let Ok(value) = toml::from_str::<toml::Value>(&content) else {
        return None;
    };
    let name = value
        .get("ai")
        .and_then(|section| section.get("framework"))
        .and_then(toml::Value::as_str)?;
    framework_from_str(name)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
