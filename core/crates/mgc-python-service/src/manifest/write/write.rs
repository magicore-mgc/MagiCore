//! Serialize shared dependencies into pyproject.toml atomically.
//! Ghi dependency dùng chung vào pyproject.toml theo cách atomic.

use std::path::Path;

use mgc_types::{Manifest, MgResult};

use super::error::PythonManifestWriteError;
use crate::manifest::dependency::python_dependency_value;
use crate::manifest::ownership::supports_native_python_dependency_groups;

pub fn write_pyproject_manifest(root: &Path, manifest: &Manifest) -> MgResult<()> {
    let path = root.join("pyproject.toml");
    let content = mgc_adapter_base::project_file::read_regular_text(&path, "pyproject.toml")?;
    let mut v: toml::Value = toml::from_str(&content).map_err(PythonManifestWriteError::Toml)?;
    if !supports_native_python_dependency_groups(&v) {
        return Err(PythonManifestWriteError::UnsupportedDependencyGroups.into());
    }
    let project = v
        .as_table_mut()
        .and_then(|t| t.get_mut("project"))
        .and_then(|p| p.as_table_mut())
        .ok_or(PythonManifestWriteError::MissingProjectTable)?;

    let deps = manifest
        .dependencies
        .iter()
        .map(python_dependency_value)
        .collect();
    project.insert("dependencies".to_string(), toml::Value::Array(deps));

    let root_table = v
        .as_table_mut()
        .ok_or(PythonManifestWriteError::InvalidRootTable)?;
    let dependency_groups = root_table
        .entry("dependency-groups")
        .or_insert_with(|| toml::Value::Table(toml::map::Map::new()))
        .as_table_mut()
        .ok_or(PythonManifestWriteError::InvalidDependencyGroupsTable)?;
    if manifest.dev_dependencies.is_empty() {
        dependency_groups.remove("dev");
    } else {
        dependency_groups.insert(
            "dev".to_string(),
            toml::Value::Array(
                manifest
                    .dev_dependencies
                    .iter()
                    .map(python_dependency_value)
                    .collect(),
            ),
        );
    }
    let remove_dependency_groups = dependency_groups.is_empty();
    if remove_dependency_groups {
        root_table.remove("dependency-groups");
    }

    let rendered = toml::to_string_pretty(&v).map_err(PythonManifestWriteError::Serialization)?;
    mgc_adapter_base::project_file::atomic_write_regular(&path, rendered.as_bytes())?;
    Ok(())
}
