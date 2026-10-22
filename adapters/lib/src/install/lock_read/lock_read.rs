//! Read and project the configured MGC lock schema for the Lib adapter.
//! Đọc và chiếu schema lock MGC đã cấu hình cho adapter Lib.

use mgc_lockfile::EcosystemTag;
use mgc_types::{MgError, MgResult, ResolvedGraph};
use std::path::Path;

use super::v4_projection::project_v4_lock_to_lib_model;
use crate::install::lock_entry_for;

pub(crate) fn read_existing_lock(project_root: &Path) -> MgResult<mgc_lockfile::Lockfile> {
    let path = project_root.join("mgc.lock");
    if mgc_config::project::lock_schema_is_v4(project_root)
        .map_err(|error| MgError::Other(error.to_string()))?
    {
        let (document, has_legacy_rows) = mgc_lockfile::migrate::read_v4_merge_base(&path)
            .map_err(|error| MgError::Other(error.to_string()))?;
        if let Some(document) = document {
            mgc_lockfile::migrate::verify_v4_merge_base(&document)
                .map_err(|error| MgError::Other(error.to_string()))?;
            return project_v4_lock_to_lib_model(project_root, &document);
        }
        // Explicit V4 configuration may be used to replace a readable V3
        // base. Keep the legacy reader for that migration boundary; a missing
        // lock remains an empty document.
        // (Schema V4 tường minh có thể thay base V3 hợp lệ; giữ reader cũ ở
        // ranh giới migrate, còn lock thiếu là document rỗng.)
        if has_legacy_rows {
            return read_existing_lock_v3(project_root);
        }
        return Ok(mgc_lockfile::Lockfile::new());
    }
    read_existing_lock_v3(project_root)
}

fn read_existing_lock_v3(project_root: &Path) -> MgResult<mgc_lockfile::Lockfile> {
    let path = project_root.join("mgc.lock");
    mgc_lockfile::parser::ensure_v3_readable_lock(&path)
        .map_err(|error| MgError::Other(error.to_string()))?;
    match mgc_lockfile::parser::load_lockfile(&path) {
        Ok(lockfile) => Ok(lockfile),
        Err(mgc_lockfile::LockfileError::IoError(error))
            if error.kind() == std::io::ErrorKind::NotFound =>
        {
            Ok(mgc_lockfile::Lockfile::new())
        }
        Err(error) => Err(MgError::Other(format!(
            "refusing to use invalid mgc.lock '{}': {error}",
            path.display()
        ))),
    }
}

pub(crate) fn validate_existing_lock_for_install(project_root: &Path) -> MgResult<()> {
    let schema_v4 = mgc_config::project::lock_schema_is_v4(project_root)
        .map_err(|error| MgError::Other(error.to_string()))?;
    if schema_v4 {
        let path = project_root.join("mgc.lock");
        let (document, _) = mgc_lockfile::migrate::read_v4_merge_base(&path)
            .map_err(|error| MgError::Other(error.to_string()))?;
        if let Some(document) = document {
            mgc_lockfile::migrate::verify_v4_merge_base(&document)
                .map_err(|error| MgError::Other(error.to_string()))?;
        }
        Ok(())
    } else {
        read_existing_lock(project_root).map(|_| ())
    }
}

pub(crate) fn needs_existing_lock_rows(
    graph: &ResolvedGraph,
    lock_packages: &[mgc_lockfile::Package],
) -> bool {
    graph
        .packages
        .iter()
        .any(|resolved| lock_entry_for(resolved, lock_packages).is_none())
}

pub(crate) fn complete_lock_packages_from_existing(
    graph: &ResolvedGraph,
    ecosystem: EcosystemTag,
    lock_packages: &mut Vec<mgc_lockfile::Package>,
    existing: &mgc_lockfile::Lockfile,
) {
    for resolved in &graph.packages {
        if lock_entry_for(resolved, lock_packages).is_some() {
            continue;
        }
        if let Some(package) = existing
            .packages
            .iter()
            .filter(|package| package.ecosystem == ecosystem)
            .find(|package| lock_entry_for(resolved, std::slice::from_ref(*package)).is_some())
        {
            lock_packages.push(package.clone());
        }
    }
}
