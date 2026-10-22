//! Parse owned project and development dependency declarations.
//! Phân tích dependency dự án và nhóm dev do MGC quản lý.

use std::path::Path;

use mgc_types::{Ecosystem, Manifest, MgResult};

use super::error::PythonManifestParseError;
use crate::manifest::dependency::parse_python_dependency;
use crate::manifest::ownership::supports_native_python_dependency_groups;

pub fn parse_pyproject_manifest(root: &Path) -> MgResult<Manifest> {
    let content = mgc_adapter_base::project_file::read_regular_text(
        &root.join("pyproject.toml"),
        "pyproject.toml",
    )?;
    let v: toml::Value = toml::from_str(&content).map_err(PythonManifestParseError::Toml)?;
    if !supports_native_python_dependency_groups(&v) {
        return Err(PythonManifestParseError::UnsupportedDependencyGroups.into());
    }
    let name = v
        .get("project")
        .and_then(|p| p.get("name"))
        .and_then(|n| n.as_str())
        .unwrap_or("unknown")
        .to_string();
    let mut manifest = Manifest::new(&name, Ecosystem::Lib);
    if let Some(deps) = v
        .get("project")
        .and_then(|p| p.get("dependencies"))
        .and_then(|d| d.as_array())
    {
        for dep in deps {
            let spec = dep.as_str().unwrap_or_default();
            let dep = parse_python_dependency(spec)?;
            manifest.add_dep(dep, false, false, false);
        }
    }
    if let Some(deps) = v
        .get("dependency-groups")
        .and_then(|groups| groups.get("dev"))
        .and_then(toml::Value::as_array)
    {
        for dep in deps {
            let spec = dep
                .as_str()
                .ok_or(PythonManifestParseError::InvalidDevelopmentRequirement)?;
            manifest.add_dep(parse_python_dependency(spec)?, true, false, false);
        }
    }
    Ok(manifest)
}
