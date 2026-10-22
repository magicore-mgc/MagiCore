//! Node package.json workspace-manifest parser.
//! Bộ phân tích manifest workspace từ package.json của Node.

use super::super::error::ManifestReadError;
use crate::graph::types::WorkspacePackageManifest;
use std::collections::HashMap;
use std::path::Path;

pub(in crate::graph::manifest_reader) fn read_package_json(
    path: &Path,
) -> Result<WorkspacePackageManifest, ManifestReadError> {
    #[derive(serde::Deserialize)]
    struct Raw {
        #[serde(rename = "name")]
        name: String,
        #[serde(default)]
        dependencies: HashMap<String, String>,
        #[serde(default, rename = "devDependencies")]
        dev_dependencies: HashMap<String, String>,
        #[serde(default, rename = "peerDependencies")]
        peer_dependencies: HashMap<String, String>,
        #[serde(default, rename = "optionalDependencies")]
        optional_dependencies: HashMap<String, String>,
    }

    let contents = std::fs::read_to_string(path)?;
    let raw: Raw = serde_json::from_str(&contents)?;
    Ok(WorkspacePackageManifest {
        name: raw.name,
        dependencies: raw.dependencies,
        dev_dependencies: raw.dev_dependencies,
        peer_dependencies: raw.peer_dependencies,
        optional_dependencies: raw.optional_dependencies,
    })
}
