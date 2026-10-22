//! App lock writer façade and scope gate.
//! Facade bộ ghi lock App và cổng kiểm tra scope.

use super::super::error::AppLockfileError;
use super::super::owner_core::app_lock_owner_core;
use super::super::v3_merge::write_v3_lane;
use super::super::v4_merge::write_v4_lane;
use mgc_lockfile::{EcosystemTag, Package};
use mgc_types::MgResult;
use std::path::Path;

/// Test-compatible entry point for the App lane.
/// Điểm vào tương thích test cho lane App.
#[cfg(test)]
pub(crate) fn write_canonical_lock(
    project_root: &Path,
    replace_ecosystems: &[EcosystemTag],
    lock_packages: Vec<Package>,
) -> MgResult<()> {
    write_canonical_lock_with_roots(project_root, replace_ecosystems, lock_packages, Vec::new())
}

/// Replace only lock entries owned by this App install lane.
/// Chỉ thay entry lock do lane install App này sở hữu.
pub(crate) fn write_canonical_lock_with_roots(
    project_root: &Path,
    replace_ecosystems: &[EcosystemTag],
    lock_packages: Vec<Package>,
    direct_package_ids: Vec<String>,
) -> MgResult<()> {
    write_canonical_lock_inner(
        project_root,
        replace_ecosystems,
        lock_packages,
        direct_package_ids,
    )
    .map_err(Into::into)
}

fn write_canonical_lock_inner(
    project_root: &Path,
    replace_ecosystems: &[EcosystemTag],
    lock_packages: Vec<Package>,
    direct_package_ids: Vec<String>,
) -> Result<(), AppLockfileError> {
    let lock_path = project_root.join("mgc.lock");
    if replace_ecosystems.is_empty() {
        return Err(AppLockfileError::message(
            "MGC_APP_LOCK_EMPTY_SCOPE",
            "validate ecosystem scope",
            "refusing to write mgc.lock without an owned ecosystem scope",
        ));
    }
    if lock_packages
        .iter()
        .any(|package| !replace_ecosystems.contains(&package.ecosystem))
    {
        return Err(AppLockfileError::message(
            "MGC_APP_LOCK_OUT_OF_SCOPE",
            "validate package ecosystem",
            "refusing to write an app lock entry outside the active ecosystem scope",
        ));
    }

    let owner_core = app_lock_owner_core(project_root)?;
    let schema_is_v4 = mgc_config::project::lock_schema_is_v4(project_root).map_err(|error| {
        AppLockfileError::caused(
            "MGC_APP_LOCK_SCHEMA_READ",
            "read lock schema",
            error.to_string(),
            error,
        )
    })?;
    if schema_is_v4 {
        return write_v4_lane(
            &lock_path,
            lock_packages,
            &owner_core,
            replace_ecosystems,
            &direct_package_ids,
        );
    }
    write_v3_lane(
        &lock_path,
        lock_packages,
        &owner_core,
        replace_ecosystems,
        &direct_package_ids,
    )
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
