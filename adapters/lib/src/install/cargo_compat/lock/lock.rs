use super::super::{project_cargo_lock, rust_packages, rust_root_ids};
use mgc_lockfile::{Lockfile, Package};
use mgc_types::MgResult;
use std::path::Path;

/// Refresh Cargo.lock from the canonical MGC lock without asking Cargo to resolve packages.
/// Cập nhật Cargo.lock từ mgc.lock canonical, không giao Cargo tự resolve package.
pub(crate) fn refresh_lock(project_root: &Path) -> MgResult<()> {
    let lock = crate::install::read_existing_lock(project_root)?;
    let rust_packages = rust_packages(&lock)?;
    let root_ids = rust_root_ids(project_root, &lock)?;
    let bytes = if rust_packages.is_empty() {
        crate::install::prepare_empty_cargo_lock(project_root, &[])?
    } else {
        Some(project_cargo_lock(project_root, &rust_packages, &root_ids)?)
    };
    if let Some(bytes) = bytes {
        crate::install::atomic_write_cargo_lock(project_root, &bytes)?;
    }
    Ok(())
}

/// Refuse unsupported graph shapes before install downloads or writes artifacts.
/// Từ chối graph ngoài phạm vi trước khi install tải hay ghi artifact.
pub(crate) fn validate_install(
    project_root: &Path,
    lock_packages: &[Package],
    graph: &mgc_types::ResolvedGraph,
) -> MgResult<()> {
    let lock = Lockfile {
        packages: lock_packages.to_vec(),
        ..Lockfile::new()
    };
    let packages = rust_packages(&lock)?;
    if packages.is_empty() {
        let _ = crate::install::prepare_empty_cargo_lock(project_root, lock_packages)?;
        return Ok(());
    }
    let roots = crate::install::direct_package_ids(graph);
    let _ = project_cargo_lock(project_root, &packages, &roots)?;
    Ok(())
}
