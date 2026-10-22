//! Verify a cached Python artifact and its importable site materialization.
//! Xác minh artifact Python trong cache và site directory có thể import.

use mgc_resolver::protocols::PypiProtocol;
use std::path::{Path, PathBuf};

use super::super::super::python_runtime::python_site_dirname;
use super::error::PythonWheelMaterializationError;

pub(in crate::install::python_runtime::path_entries) fn verify_python_package_materialization(
    package: &mgc_lockfile::Package,
    wheels_root: &Path,
    site: &Path,
    owner_core: &str,
) -> Result<PathBuf, PythonWheelMaterializationError> {
    package.integrity.strip_prefix("sha256-").ok_or_else(|| {
        PythonWheelMaterializationError::MissingIntegrity {
            name: package.name.clone(),
            version: package.version.clone(),
        }
    })?;
    let integrity = mgc_lockfile::policy::native_sha256_hex(&package.integrity);
    let integrity = integrity.as_str();
    let artifact_name = PypiProtocol::artifact_filename_for_url(&package.resolved)?;
    if !PypiProtocol::is_importable_runtime_wheel(&package.resolved)? {
        return Err(PythonWheelMaterializationError::IncompatibleRuntimeWheel {
            name: package.name.clone(),
            version: package.version.clone(),
            owner_core: owner_core.to_string(),
        });
    }
    let artifact_relative = PypiProtocol::artifact_cache_relpath(&package.resolved, integrity)?;
    let digest_artifact_path = wheels_root.join(&artifact_relative);
    // Read pre-digest caches only as a verified migration fallback.
    // (Chỉ đọc cache cũ như lối chuyển đổi có xác minh digest.)
    let artifact_path = match std::fs::symlink_metadata(&digest_artifact_path) {
        Ok(_) => digest_artifact_path,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            wheels_root.join(artifact_name)
        }
        Err(error) => {
            return Err(PythonWheelMaterializationError::from(
                mgc_types::MgError::from(error),
            ));
        }
    };
    let artifact_metadata = std::fs::symlink_metadata(&artifact_path).map_err(|source| {
        PythonWheelMaterializationError::CachedWheelUnavailable {
            name: package.name.clone(),
            version: package.version.clone(),
            source,
        }
    })?;
    if !artifact_metadata.file_type().is_file() {
        return Err(PythonWheelMaterializationError::CachedWheelNotRegularFile {
            path: artifact_path.display().to_string(),
        });
    }
    let wheel_bytes = std::fs::read(&artifact_path).map_err(mgc_types::MgError::from)?;
    let digest_site = site.join(PypiProtocol::importable_site_dirname_for_digest(
        &package.name,
        &package.version,
        integrity,
    )?);
    let legacy_site = site.join(python_site_dirname(&package.name, &package.version)?);
    let path = match std::fs::symlink_metadata(&digest_site) {
        Ok(_) => digest_site,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => legacy_site,
        Err(error) => {
            return Err(PythonWheelMaterializationError::from(
                mgc_types::MgError::from(error),
            ));
        }
    };
    PypiProtocol::verify_runtime_materialization(
        &wheel_bytes,
        &package.name,
        &package.version,
        integrity,
        &path,
    )?;
    if !std::fs::symlink_metadata(&path).is_ok_and(|meta| meta.file_type().is_dir()) {
        return Err(PythonWheelMaterializationError::MaterializationMissing {
            name: package.name.clone(),
            version: package.version.clone(),
        });
    }
    Ok(path)
}
