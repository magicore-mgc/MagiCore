//! Rust Cargo.toml workspace-manifest parser.
//! Bộ phân tích manifest workspace từ Cargo.toml của Rust.

use super::super::error::ManifestReadError;
use crate::graph::types::WorkspacePackageManifest;
use std::collections::HashMap;
use std::path::Path;

pub(in crate::graph::manifest_reader) fn read_cargo_manifest(
    path: &Path,
) -> Result<Option<WorkspacePackageManifest>, ManifestReadError> {
    #[derive(serde::Deserialize)]
    struct CargoRaw {
        package: Option<CargoPackage>,
        #[serde(default)]
        dependencies: HashMap<String, toml::Value>,
    }
    #[derive(serde::Deserialize)]
    struct CargoPackage {
        name: String,
    }

    let contents = std::fs::read_to_string(path)?;
    let Ok(raw) = toml::from_str::<CargoRaw>(&contents) else {
        return Ok(None);
    };
    let Some(package) = raw.package else {
        return Ok(None);
    };

    let mut dependencies = HashMap::new();
    for (name, value) in raw.dependencies {
        let specifier = match value {
            toml::Value::String(value) => value,
            toml::Value::Table(table) => {
                if table.contains_key("path") {
                    "workspace:*".to_string()
                } else {
                    table
                        .get("version")
                        .and_then(toml::Value::as_str)
                        .unwrap_or("*")
                        .to_string()
                }
            }
            _ => "*".to_string(),
        };
        dependencies.insert(name, specifier);
    }

    Ok(Some(WorkspacePackageManifest {
        name: package.name,
        dependencies,
        dev_dependencies: HashMap::new(),
        peer_dependencies: HashMap::new(),
        optional_dependencies: HashMap::new(),
    }))
}
