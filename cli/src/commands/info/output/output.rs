//! Human and JSON renderers for package metadata.
//! Bộ trình bày metadata dạng người đọc và JSON.

use mgc_ui::info;
use serde::Serialize;

use super::super::error::InfoError;
use mgc_web_adapter::native::npm_registry::PackageMetadata;

/// Serialize package metadata for CLI and MCP consumers.
/// Serialize metadata cho CLI và MCP.
pub(in crate::commands::info) fn render_json(
    metadata: &PackageMetadata,
    core_label: &str,
    local_version: Option<String>,
) -> Result<String, InfoError> {
    let mut versions: Vec<String> = metadata.versions.keys().cloned().collect();
    versions.sort();
    let payload = InfoJson {
        name: metadata.name.clone(),
        description: metadata.description.clone().unwrap_or_default(),
        core_support: core_label.to_string(),
        version_count: metadata.versions.len(),
        versions,
        dist_tags: metadata
            .dist_tags
            .iter()
            .map(|(name, version)| (name.clone(), version.clone()))
            .collect(),
        local_version,
    };
    serde_json::to_string_pretty(&payload).map_err(InfoError::from)
}

/// Print the existing human-readable package summary.
/// In phần tóm tắt package theo định dạng hiện tại.
pub(in crate::commands::info) fn render_human(
    meta: &PackageMetadata,
    core_label: &str,
    local_version: Option<&str>,
) {
    mgc_ui::blank_line();
    info(&format!("Package:      {}", meta.name));
    info(&format!("Core Support: {}", core_label));
    if let Some(description) = &meta.description {
        info(&format!("Description:  {}", description));
    }
    if let Some(version) = local_version {
        info(&format!("Installed:    {}", version));
    }
    info(&format!("Versions:     {}", meta.versions.len()));

    if !meta.dist_tags.is_empty() {
        mgc_ui::blank_line();
        for (tag, version) in &meta.dist_tags {
            info(&format!("  {}: {}", tag, version));
        }
    }

    let mut sorted: Vec<_> = meta.versions.keys().collect();
    sorted.sort();
    let recent: Vec<_> = sorted.iter().rev().take(5).collect();
    if !recent.is_empty() {
        mgc_ui::blank_line();
        info("Recent versions:");
        for v in recent {
            info(&format!("  {}", v));
        }
    }
}

#[derive(Serialize)]
struct InfoJson {
    name: String,
    description: String,
    core_support: String,
    version_count: usize,
    versions: Vec<String>,
    dist_tags: Vec<(String, String)>,
    local_version: Option<String>,
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
