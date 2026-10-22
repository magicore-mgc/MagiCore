//! Validate ownership and read the supported Python project manifest.
//! Xác minh quyền sở hữu và đọc manifest Python thuộc phạm vi hỗ trợ.

use mgc_types::Manifest;
use std::path::Path;

use super::super::super::python_runtime::{
    external_python_manifest_name, normalize_python_distribution_name,
};
use super::error::PythonManifestGuardError;

pub(in crate::install::python_runtime::path_entries) fn load_python_manifest(
    project_root: &Path,
) -> Result<Option<Manifest>, PythonManifestGuardError> {
    let manifest_path = project_root.join("pyproject.toml");
    let python_manifest = match std::fs::symlink_metadata(&manifest_path) {
        Ok(metadata) if metadata.file_type().is_file() => {
            if !crate::manifest::supports_native_python_project(project_root) {
                return Err(PythonManifestGuardError::UnsupportedMetadata);
            }
            Some(crate::manifest::parse_pyproject_manifest(project_root)?)
        }
        Ok(metadata) if metadata.file_type().is_symlink() => {
            return Err(PythonManifestGuardError::SymlinkedManifest);
        }
        Ok(_) => {
            return Err(PythonManifestGuardError::NonRegularManifest);
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
        Err(source) => return Err(PythonManifestGuardError::ManifestInspection { source }),
    };
    if python_manifest.is_none()
        && let Some(manifest_name) = external_python_manifest_name(project_root)?
    {
        return Err(PythonManifestGuardError::ExternalManifest { manifest_name });
    }
    Ok(python_manifest)
}

pub(in crate::install::python_runtime::path_entries) fn validate_manifest_pins(
    manifest: Option<&Manifest>,
    python_packages: &[&mgc_lockfile::Package],
    owner_core: &str,
) -> Result<(), PythonManifestGuardError> {
    if let Some(manifest) = manifest {
        for dependency in &manifest.dependencies {
            let normalized_name = normalize_python_distribution_name(dependency.name.as_str());
            let locked = python_packages.iter().find(|package| {
                normalize_python_distribution_name(&package.name) == normalized_name
            });
            let Some(locked) = locked else {
                return Err(PythonManifestGuardError::DependencyAbsentFromLock {
                    dependency: dependency.name.to_string(),
                    owner_core: owner_core.to_string(),
                });
            };
            let locked_version = mgc_types::Version::parse(&locked.version).map_err(|source| {
                PythonManifestGuardError::UncheckableLockedVersion {
                    dependency: dependency.name.to_string(),
                    locked_version: locked.version.clone(),
                    source: Box::new(source),
                }
            })?;
            if !dependency.range.matches(&locked_version) {
                return Err(PythonManifestGuardError::LockedVersionMismatch {
                    dependency: dependency.name.to_string(),
                    required: dependency.range.to_string(),
                    locked_version: locked.version.clone(),
                    owner_core: owner_core.to_string(),
                });
            }
        }
    }
    Ok(())
}
