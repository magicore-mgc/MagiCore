use super::super::source_checksum::validate_sha256;
use mgc_lockfile::{EcosystemTag, Lockfile, Package};
use mgc_types::{MgError, MgResult};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

#[path = "identity/mod.rs"]
mod identity;

use identity::is_cargo_dependency_edge_marker;
pub(in crate::install) use identity::package_id;

pub(in crate::install) fn rust_packages(lock: &Lockfile) -> MgResult<Vec<&Package>> {
    let mut packages = lock
        .packages
        .iter()
        .filter(|package| package.ecosystem == EcosystemTag::Rust)
        .collect::<Vec<_>>();
    packages.sort_by(|left, right| (&left.name, &left.version).cmp(&(&right.name, &right.version)));
    let mut identities = BTreeSet::new();
    for package in &packages {
        mgc_types::PackageName::new(package.name.clone()).map_err(|_| {
            MgError::Other(format!(
                "Cargo build refuses invalid locked crate name '{}'",
                package.name
            ))
        })?;
        mgc_types::Version::parse(&package.version).map_err(|error| {
            MgError::Other(format!(
                "Cargo build refuses invalid locked version '{}': {error}",
                package.version
            ))
        })?;
        let identity = (package.name.as_str(), package.version.as_str());
        if !identities.insert(identity) {
            return Err(MgError::Other(format!(
                "Cargo projection refuses duplicate locked package '{}@{}'",
                package.name, package.version
            )));
        }
        // The crates.io resolver records feature-optional and target-qualified
        // edges while keeping their package names in the complete Cargo lock
        // closure. Reject unknown marker types, which cannot be projected
        // without interpreting undocumented resolver metadata.
        // (Resolver crates.io giữ cạnh optional/target trong Cargo lock closure
        // đầy đủ. Từ chối kiểu marker lạ vì chưa có quy tắc projection.)
        let unsupported_marker = package.markers.as_ref().and_then(|values| {
            values
                .iter()
                .find(|marker| !is_cargo_dependency_edge_marker(marker))
        });
        if let Some(marker) = unsupported_marker {
            return Err(MgError::Other(format!(
                "Cargo projection cannot represent marker '{marker}' for '{}'@{} yet",
                package.name, package.version,
            )));
        }
        if package
            .extras
            .as_ref()
            .is_some_and(|values| !values.is_empty())
        {
            return Err(MgError::Other(format!(
                "Cargo projection cannot represent feature variants for '{}'@{} yet",
                package.name, package.version
            )));
        }
        validate_sha256(&mgc_lockfile::policy::native_sha256_hex(&package.integrity))?;
        if package.registry.as_deref().is_none_or(|registry| {
            !matches!(
                registry,
                "crates://crates.io" | "crates://sparse+https://index.crates.io"
            )
        }) {
            return Err(MgError::Other(format!(
                "Cargo projection currently supports only crates.io source identity ('{}@{}')",
                package.name, package.version
            )));
        }
    }
    Ok(packages)
}

pub(in crate::install) fn validate_build_ownership(
    packages: &[&Package],
    expected_core: &str,
) -> MgResult<()> {
    for package in packages {
        if package
            .owner_core
            .as_deref()
            .is_some_and(|owner| owner != expected_core)
        {
            return Err(MgError::Other(format!(
                "Cargo {expected_core} build cannot consume Rust package '{}' owned by core '{}'",
                package.name,
                package.owner_core.as_deref().unwrap_or_default()
            )));
        }
    }
    Ok(())
}

/// Accept only resolver marker forms whose dependency remains in the exact
/// Cargo lock closure: a valid optional crate name or a non-empty target cfg.
pub(in crate::install) fn resolve_locked_dependency(
    value: &str,
    by_id: &BTreeMap<String, &Package>,
    by_name: &BTreeMap<&str, Vec<&Package>>,
) -> MgResult<String> {
    if by_id.contains_key(value) {
        return Ok(value.to_string());
    }
    if let Some((name, version)) = value.rsplit_once('@') {
        let id = format!("{name}@{version}");
        if by_id.contains_key(&id) {
            return Ok(id);
        }
    }
    match by_name.get(value).map(Vec::as_slice).unwrap_or_default() {
        [package] => Ok(package_id(package)),
        [] => Err(MgError::Other(format!(
            "dependency '{value}' is not present in mgc.lock"
        ))),
        _ => Err(MgError::Other(format!(
            "dependency '{value}' is ambiguous across multiple locked versions"
        ))),
    }
}

pub(in crate::install) fn cargo_dependency_label(
    package: &Package,
    by_name: &BTreeMap<&str, Vec<&Package>>,
) -> String {
    if by_name
        .get(package.name.as_str())
        .is_some_and(|versions| versions.len() > 1)
    {
        format!("{} {}", package.name, package.version)
    } else {
        package.name.clone()
    }
}

pub(in crate::install) fn rust_root_ids(
    project_root: &Path,
    lock: &Lockfile,
) -> MgResult<Vec<String>> {
    let owner = crate::install::python_project_owner(project_root)?;
    let mut roots = lock
        .root_dependencies_by_owner
        .get(&owner)
        .into_iter()
        .flatten()
        .filter_map(|root| {
            let parsed = mgc_lockfile::parse_root_pin(root);
            (parsed.ecosystem == Some(EcosystemTag::Rust)).then(|| parsed.package_id.to_string())
        })
        .collect::<Vec<_>>();
    if roots.is_empty()
        && !lock
            .packages
            .iter()
            .all(|p| p.ecosystem != EcosystemTag::Rust)
    {
        roots.extend(lock.root_dependencies.iter().cloned());
    }
    roots.sort();
    roots.dedup();
    Ok(roots)
}
