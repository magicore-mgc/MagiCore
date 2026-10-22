//! v3 lock read, lane merge and publication.
//! Đọc lock v3, gộp lane và publish.

use super::super::atomic_publication::atomic_write_canonical_lock_typed;
use super::super::error::AppLockfileError;
use mgc_lockfile::{EcosystemTag, Lockfile, Package};
use std::path::Path;

pub(in crate::install::lockfile) fn write_v3_lane(
    lock_path: &Path,
    lock_packages: Vec<Package>,
    owner_core: &str,
    replace_ecosystems: &[EcosystemTag],
    direct_package_ids: &[String],
) -> Result<(), AppLockfileError> {
    mgc_lockfile::parser::ensure_v3_readable_lock(lock_path).map_err(|error| {
        AppLockfileError::caused(
            "MGC_APP_LOCK_V3_VALIDATE",
            "validate existing v3 lock",
            error.to_string(),
            error,
        )
    })?;
    let mut lockfile = match mgc_lockfile::load_lockfile(lock_path) {
        Ok(lockfile) => lockfile,
        Err(mgc_lockfile::LockfileError::IoError(error))
            if error.kind() == std::io::ErrorKind::NotFound =>
        {
            Lockfile::new()
        }
        Err(error) => {
            return Err(AppLockfileError::caused(
                "MGC_APP_LOCK_V3_READ",
                "read existing lock",
                format!("cannot read existing mgc.lock safely: {error}"),
                error,
            ));
        }
    };
    let original_packages = lockfile.packages.clone();
    let original_version = lockfile.version.clone();
    let original_roots = lockfile.root_dependencies_by_owner.clone();

    // A shared ecosystem may have rows owned by another core.
    // Một ecosystem dùng chung có thể có row thuộc core khác.
    for ecosystem in replace_ecosystems {
        let competing_owner_exists = lockfile.packages.iter().any(|package| {
            package.ecosystem == *ecosystem
                && package
                    .owner_core
                    .as_deref()
                    .is_some_and(|owner| owner != owner_core)
        });
        if !competing_owner_exists {
            for package in &mut lockfile.packages {
                if package.ecosystem == *ecosystem && package.owner_core.is_none() {
                    package.owner_core = Some(owner_core.to_string());
                }
            }
        }
    }
    let mut owned_packages = lock_packages;
    for package in &mut owned_packages {
        package.owner_core = Some(owner_core.to_string());
    }
    lockfile.packages.retain(|package| {
        !replace_ecosystems.contains(&package.ecosystem)
            || package.owner_core.as_deref() != Some(owner_core)
    });
    lockfile.packages.extend(owned_packages);
    for ecosystem in replace_ecosystems {
        mgc_lockfile::update_owner_root_pins(
            &mut lockfile,
            owner_core,
            *ecosystem,
            direct_package_ids.to_vec(),
        );
    }
    if lock_path.is_file()
        && lockfile.packages == original_packages
        && lockfile.root_dependencies_by_owner == original_roots
        && original_version == mgc_lockfile::LOCKFILE_SCHEMA_VERSION
    {
        return Ok(());
    }
    mgc_lockfile::ensure_lockfile_mutation_allowed(lock_path).map_err(|error| {
        AppLockfileError::caused(
            "MGC_APP_LOCK_MUTATION_DENIED",
            "authorize lock mutation",
            error.to_string(),
            error,
        )
    })?;
    lockfile.metadata.generated_at = chrono::Utc::now().to_rfc3339();
    lockfile.metadata.generator = format!("mgc/{}", env!("CARGO_PKG_VERSION"));
    let toml = mgc_lockfile::writer::serialize_lockfile(&lockfile).map_err(|error| {
        AppLockfileError::caused(
            "MGC_APP_LOCK_V3_SERIALIZE",
            "serialize v3 lock",
            format!("lockfile serialization failed: {error}"),
            error,
        )
    })?;
    atomic_write_canonical_lock_typed(lock_path, toml.as_bytes())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
