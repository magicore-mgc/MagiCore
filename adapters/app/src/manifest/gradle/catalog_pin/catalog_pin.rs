//! Safe single-pin mutation for Gradle version catalogs.
//! Thay đổi an toàn một pin trong version catalog Gradle.

use std::path::Path;

use mgc_adapter_base::project_file::{atomic_write_project_regular, read_project_regular_text};

use super::super::catalog_reader::parse_catalog_content;
use super::super::error::GradleManifestError;
use super::super::version_ref_uses::count_version_ref_uses;

/// Update a catalog pin and fail closed if the serialized value does not read back.
/// Cập nhật pin catalog và fail-closed nếu giá trị ghi ra không đọc lại được.
pub(in crate::manifest::gradle) fn bump_catalog_pin(
    project_root: &Path,
    group: &str,
    artifact: &str,
    new_version: &str,
) -> Result<bool, GradleManifestError> {
    let content = read_project_regular_text(
        project_root,
        Path::new("gradle/libs.versions.toml"),
        "libs.versions.toml",
    )?;
    let entries = parse_catalog_content(&content).ok_or_else(|| {
        GradleManifestError::Other("unsupported libs.versions.toml shape".to_string())
    })?;
    let hit = entries
        .iter()
        .find(|entry| entry.group == group && entry.artifact == artifact);
    let Some(hit) = hit else {
        return Ok(false);
    };
    if hit.version == new_version {
        return Ok(false);
    }
    let mut editable: toml_edit::DocumentMut = content.parse().map_err(|error| {
        GradleManifestError::Other(format!("parse editable libs.versions.toml: {error}"))
    })?;
    if let Some(reference) = &hit.version_ref {
        let source: toml::Value = toml::from_str(&content).map_err(|error| {
            GradleManifestError::Other(format!("parse libs.versions.toml: {error}"))
        })?;
        if count_version_ref_uses(&source, reference) > 1 {
            return Err(GradleManifestError::Unsupported {
                core: "app",
                capability: "bump-shared-gradle-version-ref",
                guidance: format!(
                    concat!(
                        "version ref '{}' is shared by multiple catalog entries; ",
                        "refusing to change unrelated dependencies through a single-package update"
                    ),
                    reference
                ),
            });
        }
        // Rewrite the single [versions] value — sửa giá trị duy nhất tại [versions].
        editable["versions"][reference] = toml_edit::value(new_version);
    } else {
        // Rewrite only the structured library value — chỉ sửa giá trị có cấu trúc của library.
        let library = &mut editable["libraries"][&hit.alias];
        if let Some(value) = library.as_value_mut().filter(|value| value.is_str()) {
            let mut replacement =
                toml_edit::Value::from(format!("{group}:{artifact}:{new_version}"));
            *replacement.decor_mut() = value.decor().clone();
            *value = replacement;
        } else if library.is_inline_table() {
            library["version"] = toml_edit::value(new_version);
        } else {
            return Err(GradleManifestError::Other(format!(
                "catalog pin {group}:{artifact} has unsupported editable shape"
            )));
        }
    }
    let next = editable.to_string();
    atomic_write_project_regular(
        project_root,
        Path::new("gradle/libs.versions.toml"),
        next.as_bytes(),
    )?;
    let verified = read_project_regular_text(
        project_root,
        Path::new("gradle/libs.versions.toml"),
        "libs.versions.toml",
    )?;
    let again = parse_catalog_content(&verified).unwrap_or_default();
    let verified_pin = again.iter().any(|entry| {
        entry.group == group && entry.artifact == artifact && entry.version == new_version
    });
    if !verified_pin {
        return Err(GradleManifestError::Other(format!(
            "catalog rewrite did not stick for {group}:{artifact} (fail-closed)"
        )));
    }
    Ok(true)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
