//! Resolve project ownership and select only matching Python lock entries.
//! Xác định owner project và chỉ chọn entry Python trong lock tương ứng.

use std::path::Path;

use mgc_lockfile::{EcosystemTag, Package};

use super::error::PythonCoreOwnerError;

pub(in crate::install) fn python_project_owner(
    project_root: &Path,
) -> Result<String, PythonCoreOwnerError> {
    if let Some(core) = mgc_config::project::ProjectConfig::read_core_marker(project_root)
        .map_err(PythonCoreOwnerError::ReadMarker)?
    {
        return Ok(canonical_core_name(&core));
    }
    if let Some(config) = mgc_config::project::ProjectConfig::load(project_root)
        .map_err(PythonCoreOwnerError::ReadConfig)?
    {
        return Ok(canonical_core_name(&config.ecosystem));
    }
    Ok("lib".to_string())
}

pub(in crate::install) fn select_python_packages_for_owner<'a>(
    packages: &'a [Package],
    owner_core: &str,
) -> Result<Vec<&'a Package>, PythonCoreOwnerError> {
    let all_python_packages = packages
        .iter()
        .filter(|package| package.ecosystem == EcosystemTag::Python)
        .collect::<Vec<_>>();
    let has_unowned_packages = all_python_packages
        .iter()
        .any(|package| package.owner_core.is_none());
    if owner_core != "lib" && has_unowned_packages {
        return Err(PythonCoreOwnerError::AmbiguousLegacyOwnership);
    }
    Ok(all_python_packages
        .into_iter()
        .filter(|package| {
            package.owner_core.as_deref() == Some(owner_core)
                || (owner_core == "lib" && package.owner_core.is_none())
        })
        .collect())
}

pub(in crate::install) fn canonical_core_name(value: &str) -> String {
    let core = value.trim().to_ascii_lowercase();
    if core == "cloud" {
        "clo".to_string()
    } else {
        core
    }
}
