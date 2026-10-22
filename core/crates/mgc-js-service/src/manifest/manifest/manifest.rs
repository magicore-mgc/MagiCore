//! Parse and update the shared package.json manifest projection.
//! Đọc và cập nhật projection manifest package.json dùng chung.

use crate::manifest::error::ManifestError;
use crate::manifest::package_json::PackageJson;
use crate::manifest::package_json::load_checked;
use crate::manifest::workspace_protocol::is_workspace_protocol_range;
use mgc_types::{DependencySpec, Manifest, MgError, MgResult, PackageName, Version, VersionRange};
use std::path::Path;

/// Read package.json as the shared Web-compatible manifest projection.
/// Đọc package.json thành projection manifest tương thích Web dùng chung.
pub fn parse_manifest(project_root: &Path) -> MgResult<Manifest> {
    let pkg_path = project_root.join("package.json");
    let pkg_json = match load_checked(&pkg_path) {
        Ok(package_json) => package_json,
        Err(MgError::Io(source)) if source.kind() == std::io::ErrorKind::NotFound => {
            return Err(ManifestError::MissingPackageJson {
                project_root: project_root.to_path_buf(),
            }
            .into());
        }
        Err(error) => return Err(error),
    };
    let mut manifest = Manifest::new(&pkg_json.name, mgc_types::ecosystem::Ecosystem::Web);
    manifest.version = if pkg_json.version.is_empty() {
        None
    } else {
        Some(Version::parse(&pkg_json.version).map_err(|_| {
            MgError::from(ManifestError::InvalidVersion {
                version: pkg_json.version.clone(),
            })
        })?)
    };
    let parse_deps =
        |map: Option<std::collections::HashMap<String, String>>| -> MgResult<Vec<DependencySpec>> {
            match map {
                Some(deps) => {
                    let mut out = Vec::with_capacity(deps.len());
                    for (name, range) in deps {
                        if is_workspace_protocol_range(&range) {
                            continue;
                        }
                        let package_name = PackageName::new(name)?;
                        let version_range = VersionRange::parse(&range)?;
                        out.push(DependencySpec::new(package_name, version_range));
                    }
                    Ok(out)
                }
                None => Ok(vec![]),
            }
        };
    manifest.dependencies = parse_deps(pkg_json.dependencies)?;
    manifest.dev_dependencies = parse_deps(pkg_json.dev_dependencies)?;
    manifest.peer_dependencies = parse_deps(pkg_json.peer_dependencies)?;
    manifest.optional_dependencies = parse_deps(pkg_json.optional_dependencies)?;
    Ok(manifest)
}

/// Update package.json without discarding fields outside MagiCore's graph.
/// Cập nhật package.json mà không xóa field ngoài graph của MagiCore.
pub fn write_manifest(project_root: &Path, manifest: &Manifest) -> MgResult<()> {
    let to_map = |dependencies: &[DependencySpec]| -> std::collections::HashMap<String, String> {
        dependencies
            .iter()
            .map(|dependency| {
                (
                    dependency.name.as_str().to_string(),
                    dependency.range.to_string(),
                )
            })
            .collect()
    };
    let pkg_path = project_root.join("package.json");
    let fallback_version = manifest
        .version
        .as_ref()
        .map(|version| version.to_string())
        .unwrap_or_else(|| "0.1.0".to_string());
    let existing = match std::fs::symlink_metadata(&pkg_path) {
        Ok(_) => PackageJson::load(&pkg_path)
            .map_err(|source| MgError::from(ManifestError::ExistingPackageRead { source }))?,
        Err(source) if source.kind() == std::io::ErrorKind::NotFound => {
            PackageJson::new(manifest.name.clone(), fallback_version)
        }
        Err(source) => {
            return Err(ManifestError::InspectPackageJson { source }.into());
        }
    };
    let merge_workspace = |dependencies: &[DependencySpec],
                           old: Option<&std::collections::HashMap<String, String>>|
     -> MgResult<Option<std::collections::HashMap<String, String>>> {
        let mut entries = to_map(dependencies);
        if let Some(old) = old {
            for (name, range) in old {
                if is_workspace_protocol_range(range) {
                    if entries.contains_key(name) {
                        return Err(ManifestError::WorkspaceDependencyReplacement {
                            name: name.clone(),
                        }
                        .into());
                    }
                    entries.entry(name.clone()).or_insert_with(|| range.clone());
                }
            }
        }
        Ok((!entries.is_empty()).then_some(entries))
    };
    let package_json = PackageJson {
        name: manifest.name.clone(),
        version: manifest
            .version
            .as_ref()
            .map(|version| version.to_string())
            .unwrap_or(existing.version),
        description: existing.description,
        dependencies: merge_workspace(&manifest.dependencies, existing.dependencies.as_ref())?,
        dev_dependencies: merge_workspace(
            &manifest.dev_dependencies,
            existing.dev_dependencies.as_ref(),
        )?,
        peer_dependencies: merge_workspace(
            &manifest.peer_dependencies,
            existing.peer_dependencies.as_ref(),
        )?,
        optional_dependencies: merge_workspace(
            &manifest.optional_dependencies,
            existing.optional_dependencies.as_ref(),
        )?,
        extra: existing.extra,
    };
    package_json.save(&pkg_path)?;
    Ok(())
}
