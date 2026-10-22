//! Merge and atomically publish canonical MGC lock lanes.
//! Gộp và ghi atomic các lane lock MGC canonical.

use mgc_lockfile::EcosystemTag;
use mgc_types::{MgError, MgResult, ResolvedGraph};
use std::path::Path;

use super::{
    atomic_publication::atomic_write_canonical_lock, v4_lane_write::write_canonical_lock_v4,
};
use crate::install::read_existing_lock;

/// Flush the native-resolution lock entries to the canonical mgc.lock v3.
/// Ghi entry lock từ resolve native xuống mgc.lock v3 canonical.
#[cfg(test)]
pub(crate) fn write_canonical_lock(
    project_root: &Path,
    ecosystem: EcosystemTag,
    lock_packages: Vec<mgc_lockfile::Package>,
) -> MgResult<()> {
    write_canonical_lock_with_roots(project_root, ecosystem, lock_packages, Vec::new())
}

pub(crate) fn write_canonical_lock_with_roots(
    project_root: &Path,
    ecosystem: EcosystemTag,
    mut lock_packages: Vec<mgc_lockfile::Package>,
    direct_package_ids: Vec<String>,
) -> MgResult<()> {
    let lock_path = project_root.join("mgc.lock");
    if lock_packages.iter().any(|pkg| pkg.ecosystem != ecosystem) {
        return Err(MgError::Other(format!(
            "resolver returned a package outside the active ecosystem '{ecosystem}'"
        )));
    }
    let marker = mgc_config::project::ProjectConfig::read_core_marker(project_root)
        .map_err(|error| MgError::Other(format!("cannot determine lock owner core: {error}")))?;
    let (owner_core, explicit_project_owner) = match marker {
        Some(core) => (core, true),
        None => match mgc_config::project::ProjectConfig::load(project_root)
            .map_err(|error| MgError::Other(format!("cannot read project core owner: {error}")))?
        {
            Some(config) => (canonical_core_name(&config.ecosystem), true),
            None => ("lib".to_string(), false),
        },
    };
    if !mgc_config::project::ProjectConfig::KNOWN_CORES.contains(&owner_core.as_str()) {
        return Err(MgError::Other(format!(
            "refusing to write lock entries for unknown core owner '{owner_core}'"
        )));
    }

    // v4 write path (explicit `[lock] schema` only): lane rows convert
    // losslessly against the v4 base on disk — the legacy v3 model is never
    // in the loop, so a v4 lock stays readable across installs.
    // (Đường ghi v4 (chỉ khi `[lock] schema` tường minh): row lane convert
    // với base v4 trên đĩa — model v3 legacy không tham gia, nên lock v4
    // đọc được qua mọi lần install.)
    if mgc_config::project::lock_schema_is_v4(project_root)
        .map_err(|error| MgError::Other(error.to_string()))?
    {
        return write_canonical_lock_v4(
            &lock_path,
            lock_packages,
            &owner_core,
            &[ecosystem],
            &direct_package_ids,
        );
    }

    // V3 reader is intentionally below the schema dispatch: feeding V4
    // through the legacy model breaks frozen replay even though the V4
    // merge writer already knows how to preserve it.
    // (Reader V3 nằm sau nhánh schema: parse V4 bằng model legacy làm hỏng
    // frozen replay dù writer merge V4 đã hỗ trợ.)
    let mut lockfile = read_existing_lock(project_root)?;
    let original_packages = lockfile.packages.clone();
    let original_version = lockfile.version.clone();
    let original_roots = lockfile.root_dependencies_by_owner.clone();
    // Replace this ecosystem's resolved set as a whole. Package-only merge
    // retained removed and transitive dependencies in the unified lock.
    // (Thay trọn tập ecosystem; merge từng package để lại entry đã gỡ.)

    // A unified lock may contain the same language ecosystem under multiple
    // cores (for example AI/Python and Lib/Python). Replace only this core's
    // entries. A single-core project signature permits safe adoption of a
    // legacy unowned set; a mixed lock with a competing owner stays ambiguous
    // and is preserved instead of guessed or deleted.
    // (Lock hợp nhất có thể dùng chung ecosystem ở nhiều core; chỉ thay entry
    // của core hiện tại; chỉ nhận diện lock cũ khi chữ ký project không mơ hồ.)
    let conflicting_owner_exists = lockfile.packages.iter().any(|pkg| {
        pkg.ecosystem == ecosystem
            && pkg
                .owner_core
                .as_deref()
                .is_some_and(|owner| owner != owner_core)
    });
    if explicit_project_owner && !conflicting_owner_exists {
        for package in &mut lockfile.packages {
            if package.ecosystem == ecosystem && package.owner_core.is_none() {
                package.owner_core = Some(owner_core.clone());
            }
        }
    }
    for package in &mut lock_packages {
        package.owner_core = Some(owner_core.clone());
    }
    lockfile.packages.retain(|pkg| {
        pkg.ecosystem != ecosystem || pkg.owner_core.as_deref() != Some(owner_core.as_str())
    });
    lockfile.packages.extend(lock_packages);
    mgc_lockfile::update_owner_root_pins(
        &mut lockfile,
        &owner_core,
        ecosystem,
        direct_package_ids.clone(),
    );
    if lock_path.is_file()
        && lockfile.packages == original_packages
        && lockfile.root_dependencies_by_owner == original_roots
        && original_version == mgc_lockfile::LOCKFILE_SCHEMA_VERSION
    {
        return Ok(());
    }
    mgc_lockfile::ensure_lockfile_mutation_allowed(&lock_path)
        .map_err(|error| MgError::Other(error.to_string()))?;
    lockfile.metadata.generated_at = chrono::Utc::now().to_rfc3339();
    lockfile.metadata.generator = format!("mgc/{}", env!("CARGO_PKG_VERSION"));
    let toml = mgc_lockfile::writer::serialize_lockfile(&lockfile)
        .map_err(|e| MgError::Other(format!("lockfile serialization failed: {e}")))?;
    atomic_write_canonical_lock(&lock_path, toml.as_bytes())
}

pub(crate) fn direct_package_ids(graph: &ResolvedGraph) -> Vec<String> {
    graph
        .packages
        .iter()
        .filter(|package| package.direct)
        .map(|package| package.id.to_string())
        .collect()
}

// COMPAT facade (S09a/P03) — delegates to mgc_python_service::install.
// Facade tương thích (S09a/P03) — ủy quyền cho mgc_python_service.
// Callers: python_project_owner + write_canonical_lock_with_roots (generic
//   lock writer; body stays in lib).
// TODO(S09b-cleanup): remove once ai retargets onto mgc-python-service directly.
fn canonical_core_name(value: &str) -> String {
    mgc_python_service::install::canonical_core_name(value)
}
