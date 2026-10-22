//! Existing CI/CD manifest classification.
//! Phân loại manifest CI/CD hiện hành.

use std::path::Path;

use crate::provider::detect_provider;

/// Classify a project as CI/CD using the existing config and marker rules.
/// Phân loại project CI/CD theo quy tắc config và marker hiện hành.
pub(crate) fn manifest_is_cicd(root: &Path) -> bool {
    if let Ok(Some(content)) =
        mgc_config::project::read_regular_project_text(&root.join("mgc.toml"), "project config")
        && let Ok(value) = toml::from_str::<toml::Value>(&content)
    {
        if value
            .get("ecosystem")
            .and_then(|ecosystem| ecosystem.as_str())
            == Some("cicd")
        {
            return true;
        }
        if value.get("cicd").is_some() {
            return true;
        }
    }
    detect_provider(root).is_some()
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
