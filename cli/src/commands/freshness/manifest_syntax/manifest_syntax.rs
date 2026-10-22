//! Validates the manifest structures currently accepted by freshness gates; it does not broaden parsing. — Xác thực cấu trúc manifest hiện được freshness gate chấp nhận; không mở rộng parsing.

use anyhow::{Context, Result};

/// Validate syntax for the manifest formats with structural freshness checks.
/// Xác thực cú pháp cho các dạng manifest có structural freshness check.
pub(in crate::commands::freshness) fn validate_manifest_syntax(
    file: &str,
    content: &str,
) -> Result<()> {
    match file {
        "package.json" => {
            let value = serde_json::from_str::<serde_json::Value>(content)
                .context("package.json is not valid JSON")?;
            let Some(root) = value.as_object() else {
                return Err(crate::error::freshness_manifest_invalid(
                    file,
                    "must contain a JSON object at the root",
                ));
            };
            for section in [
                "dependencies",
                "devDependencies",
                "peerDependencies",
                "optionalDependencies",
            ] {
                if root.get(section).is_some_and(|value| !value.is_object()) {
                    return Err(crate::error::freshness_manifest_invalid(
                        file,
                        &format!("field '{section}' must be an object"),
                    ));
                }
            }
        }
        "pyproject.toml" => {
            let value = toml::from_str::<toml::Value>(content)
                .with_context(|| format!("{file} is not valid TOML"))?;
            if let Some(project) = value.get("project") {
                let Some(project) = project.as_table() else {
                    return Err(crate::error::freshness_manifest_invalid(
                        file,
                        "field 'project' must be a table",
                    ));
                };
                if let Some(dependencies) = project.get("dependencies")
                    && dependencies
                        .as_array()
                        .is_none_or(|items| items.iter().any(|item| item.as_str().is_none()))
                {
                    return Err(crate::error::freshness_manifest_invalid(
                        file,
                        "field 'project.dependencies' must be an array of strings",
                    ));
                }
                if let Some(groups) = project.get("optional-dependencies") {
                    let Some(groups) = groups.as_table() else {
                        return Err(crate::error::freshness_manifest_invalid(
                            file,
                            "field 'project.optional-dependencies' must be a table",
                        ));
                    };
                    if groups.values().any(|items| {
                        items
                            .as_array()
                            .is_none_or(|items| items.iter().any(|item| item.as_str().is_none()))
                    }) {
                        return Err(crate::error::freshness_manifest_invalid(
                            file,
                            "optional dependency groups must be arrays of strings",
                        ));
                    }
                }
            }
        }
        "Cargo.toml" => {
            let value = toml::from_str::<toml::Value>(content)
                .with_context(|| format!("{file} is not valid TOML"))?;
            for section in ["dependencies", "dev-dependencies", "build-dependencies"] {
                if value.get(section).is_some_and(|value| !value.is_table()) {
                    return Err(crate::error::freshness_manifest_invalid(
                        file,
                        &format!("field '{section}' must be a table"),
                    ));
                }
            }
        }
        "pubspec.yaml" => {
            let value = serde_yaml::from_str::<serde_yaml::Value>(content)
                .context("pubspec.yaml is not valid YAML")?;
            if !value.is_mapping() {
                return Err(crate::error::freshness_manifest_invalid(
                    file,
                    "must contain a YAML mapping at the root",
                ));
            }
            for section in ["dependencies", "dev_dependencies"] {
                if value.get(section).is_some_and(|value| !value.is_mapping()) {
                    return Err(crate::error::freshness_manifest_invalid(
                        file,
                        &format!("field '{section}' must be a mapping"),
                    ));
                }
            }
        }
        _ => {}
    }
    Ok(())
}
