//! Python pyproject.toml workspace-manifest parser.
//! Bộ phân tích manifest workspace từ pyproject.toml của Python.

use super::super::error::ManifestReadError;
use crate::graph::types::WorkspacePackageManifest;
use std::collections::HashMap;
use std::path::Path;

pub(in crate::graph::manifest_reader) fn read_pyproject_manifest(
    path: &Path,
) -> Result<Option<WorkspacePackageManifest>, ManifestReadError> {
    #[derive(serde::Deserialize)]
    struct PyRaw {
        project: Option<PyProject>,
    }
    #[derive(serde::Deserialize)]
    struct PyProject {
        name: String,
    }

    let contents = std::fs::read_to_string(path)?;
    let Ok(raw) = toml::from_str::<PyRaw>(&contents) else {
        return Ok(None);
    };
    let Some(project) = raw.project else {
        return Ok(None);
    };

    Ok(Some(WorkspacePackageManifest {
        name: project.name,
        dependencies: HashMap::new(),
        dev_dependencies: HashMap::new(),
        peer_dependencies: HashMap::new(),
        optional_dependencies: HashMap::new(),
    }))
}
