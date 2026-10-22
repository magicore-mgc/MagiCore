//! Public crate-local routes for including SDK package dependencies.
//! Các route nội bộ crate để đưa dependency package SDK vào graph.

use crate::manifest::flutter::FlutterResult;
use mgc_types::Manifest;
use std::path::{Path, PathBuf};

/// Expand dependencies declared by Flutter SDK packages into the Pub graph.
/// The SDK supplies package source; MGC still resolves registry packages.
/// Mở rộng dependency của package Flutter SDK vào graph Pub; MGC vẫn tự resolve package registry.
pub(crate) fn expand_flutter_sdk_dependencies(
    project_root: &Path,
    manifest: &Manifest,
) -> FlutterResult<Manifest> {
    let declared = super::super::manifest_closure::declared_flutter_sdk_dependencies(project_root)?;
    if declared.is_empty() {
        return Ok(manifest.clone());
    }
    let sdk_root = super::super::discovery::discover_flutter_sdk_root()?;
    let (_, expanded) = super::super::manifest_closure::read_flutter_sdk_dependency_closure(
        project_root,
        &sdk_root,
        manifest,
        declared,
    )?;
    Ok(expanded)
}

pub(crate) fn flutter_sdk_packages(
    project_root: &Path,
) -> FlutterResult<(Option<PathBuf>, Vec<String>)> {
    if super::super::manifest_closure::declared_flutter_sdk_dependencies(project_root)?.is_empty() {
        return Ok((None, Vec::new()));
    }
    let sdk_root = super::super::discovery::discover_flutter_sdk_root()?;
    let packages =
        super::super::manifest_closure::flutter_sdk_package_names(project_root, &sdk_root)?;
    Ok((Some(sdk_root), packages))
}

pub(crate) fn has_flutter_sdk_dependencies(project_root: &Path) -> FlutterResult<bool> {
    Ok(
        !super::super::manifest_closure::declared_flutter_sdk_dependencies(project_root)?
            .is_empty(),
    )
}

#[cfg(test)]
pub(crate) fn expand_flutter_sdk_dependencies_with_root(
    project_root: &Path,
    manifest: &Manifest,
    sdk_root: &Path,
) -> FlutterResult<Manifest> {
    let declared = super::super::manifest_closure::declared_flutter_sdk_dependencies(project_root)?;
    if declared.is_empty() {
        return Ok(manifest.clone());
    }
    let (names, expanded) = super::super::manifest_closure::read_flutter_sdk_dependency_closure(
        project_root,
        sdk_root,
        manifest,
        declared,
    )?;
    let _ = names;
    Ok(expanded)
}
