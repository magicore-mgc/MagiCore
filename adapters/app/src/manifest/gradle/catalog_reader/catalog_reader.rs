//! Reader and parser for Gradle version catalogs.
//! Bộ đọc và parser version catalog Gradle.

use std::path::Path;

use mgc_adapter_base::project_file::read_project_regular_text;

use super::super::catalog_entry::CatalogEntry;

/// Parse `gradle/libs.versions.toml`, returning `None` when unavailable or unsupported.
/// Parse `gradle/libs.versions.toml`, trả `None` nếu thiếu hoặc chưa hỗ trợ.
pub(in crate::manifest::gradle) fn parse_version_catalog(
    project_root: &Path,
) -> Option<Vec<CatalogEntry>> {
    let content = read_project_regular_text(
        project_root,
        Path::new("gradle/libs.versions.toml"),
        "libs.versions.toml",
    )
    .ok()?;
    parse_catalog_content(&content)
}

pub(in crate::manifest::gradle) fn parse_catalog_content(
    content: &str,
) -> Option<Vec<CatalogEntry>> {
    let doc: toml::Value = toml::from_str(content).ok()?;
    let versions = doc.get("versions")?.as_table()?;
    let libraries = doc.get("libraries")?.as_table()?;
    let mut out = Vec::new();
    for (alias, spec) in libraries {
        let (module, version, version_ref): (String, String, Option<String>) = match spec {
            toml::Value::String(value) => {
                let mut parts = value.rsplitn(2, ':');
                let (Some(version), Some(module)) = (parts.next(), parts.next()) else {
                    continue;
                };
                if version.is_empty() || module.is_empty() {
                    continue;
                }
                (module.to_string(), version.to_string(), None)
            }
            toml::Value::Table(table) => {
                let Some(module) = table.get("module").and_then(|value| value.as_str()) else {
                    continue;
                };
                match table.get("version") {
                    Some(toml::Value::String(version)) => {
                        (module.to_string(), version.clone(), None)
                    }
                    Some(other) => {
                        let Some(reference) = other.get("ref").and_then(|value| value.as_str())
                        else {
                            continue;
                        };
                        let Some(version) =
                            versions.get(reference).and_then(|value| value.as_str())
                        else {
                            continue;
                        };
                        (
                            module.to_string(),
                            version.to_string(),
                            Some(reference.to_string()),
                        )
                    }
                    None => continue,
                }
            }
            _ => continue,
        };
        let mut parts = module.splitn(2, ':');
        let (Some(group), Some(artifact)) = (parts.next(), parts.next()) else {
            continue;
        };
        if group.is_empty() || artifact.is_empty() {
            continue;
        }
        out.push(CatalogEntry {
            alias: alias.clone(),
            group: group.to_string(),
            artifact: artifact.to_string(),
            version,
            version_ref,
        });
    }
    Some(out)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
