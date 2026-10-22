use super::super::{
    CARGO_LOCK_HEADER, CARGO_LOCK_VERSION, CRATES_IO_SOURCE, CargoLock, CargoPackage,
    cargo_dependency_label, package_id, read_project_manifest, resolve_locked_dependency,
    resolve_manifest_roots,
};
use mgc_lockfile::Package;
use mgc_types::{MgError, MgResult};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::path::Path;

pub(in crate::install) fn project_cargo_lock(
    project_root: &Path,
    packages: &[&Package],
    root_ids: &[String],
) -> MgResult<Vec<u8>> {
    let cargo_manifest = read_project_manifest(project_root)?;
    let manifest = &cargo_manifest.document;
    let root_name = cargo_manifest.root_name.as_str();
    let root_version = cargo_manifest.root_version.as_str();

    let mut by_id = BTreeMap::<String, &Package>::new();
    let mut by_name = BTreeMap::<&str, Vec<&Package>>::new();
    for package in packages {
        by_id.insert(package_id(package), package);
        by_name
            .entry(package.name.as_str())
            .or_default()
            .push(package);
    }
    if by_name.contains_key(root_name) {
        return Err(MgError::Other(
            "Cargo root package name collides with a registry package".to_string(),
        ));
    }
    let roots = resolve_manifest_roots(manifest, root_ids, &by_id, &by_name)?;

    validate_reachable_lock_graph(root_ids, packages, &by_id, &by_name)?;

    let mut rows = Vec::with_capacity(packages.len() + 1);
    rows.push(CargoPackage {
        name: root_name.to_string(),
        version: root_version.to_string(),
        source: None,
        checksum: None,
        dependencies: roots,
    });
    for package in packages {
        let mut dependencies = package
            .dependencies
            .iter()
            .map(|dependency| resolve_locked_dependency(dependency, &by_id, &by_name))
            .collect::<MgResult<Vec<_>>>()?
            .iter()
            .map(|id| {
                by_id
                    .get(id)
                    .map(|target| cargo_dependency_label(target, &by_name))
                    .ok_or_else(|| MgError::Other(format!("missing Rust package '{id}'")))
            })
            .collect::<MgResult<Vec<_>>>()?;
        dependencies.sort();
        dependencies.dedup();
        rows.push(CargoPackage {
            name: package.name.clone(),
            version: package.version.clone(),
            source: Some(CRATES_IO_SOURCE.to_string()),
            checksum: Some(mgc_lockfile::policy::native_sha256_hex(&package.integrity)),
            dependencies,
        });
    }
    rows.sort_by(|left, right| {
        (&left.name, &left.version, &left.source).cmp(&(&right.name, &right.version, &right.source))
    });
    let document = CargoLock {
        version: CARGO_LOCK_VERSION,
        package: rows,
    };
    let serialized = toml::to_string(&document)
        .map_err(|error| MgError::Other(format!("serialize Cargo.lock projection: {error}")))?;
    let mut bytes = CARGO_LOCK_HEADER.as_bytes().to_vec();
    bytes.extend_from_slice(serialized.as_bytes());
    Ok(bytes)
}

/// Reject lock graph rows that cannot be reached from the declared direct roots.
/// Từ chối package lock không thể đi tới từ các root dependency đã khai báo.
pub(in crate::install) fn validate_reachable_lock_graph(
    root_ids: &[String],
    packages: &[&Package],
    by_id: &BTreeMap<String, &Package>,
    by_name: &BTreeMap<&str, Vec<&Package>>,
) -> MgResult<()> {
    let mut reachable = BTreeSet::<String>::new();
    let mut pending = VecDeque::from(root_ids.to_vec());
    while let Some(name) = pending.pop_front() {
        if !reachable.insert(name.clone()) {
            continue;
        }
        let package = by_id.get(name.as_str()).ok_or_else(|| {
            MgError::Other(format!(
                "mgc.lock dependency '{name}' is not present in its package table"
            ))
        })?;
        for dependency in &package.dependencies {
            pending.push_back(
                resolve_locked_dependency(dependency, by_id, by_name).map_err(|error| {
                    MgError::Other(format!(
                        "mgc.lock package '{}'@{} has invalid dependency edge: {error}",
                        package.name, package.version
                    ))
                })?,
            );
        }
    }
    if reachable.len() != packages.len() {
        return Err(MgError::Other(
            "mgc.lock contains Rust packages unreachable from Cargo.toml roots".to_string(),
        ));
    }
    Ok(())
}
