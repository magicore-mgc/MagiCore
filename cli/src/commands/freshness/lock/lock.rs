//! Owns lock reads, owner/ecosystem root projection and exact declared-versus-locked comparison. — Sở hữu đọc lock, chiếu root theo owner/ecosystem và so khớp khai báo với lock.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use anyhow::Result;

/// Read package identities from either supported lock schema.
/// Đọc danh tính package từ cả hai schema lock được hỗ trợ.
pub(in crate::commands::freshness) fn locked_packages(
    project_root: &Path,
) -> Result<Vec<(String, String)>> {
    let lock_path = project_root.join("mgc.lock");
    let document = mgc_lockfile::parser::load_lock_document(&lock_path).map_err(|e| {
        crate::error::stale_lock_needs_install(&format!("cannot read mgc.lock: {e}"))
    })?;
    Ok(match document {
        mgc_lockfile::LockDocument::Legacy(lockfile) => lockfile
            .packages
            .into_iter()
            .map(|package| (package.name, package.version))
            .collect(),
        mgc_lockfile::LockDocument::V4(lockfile) => lockfile
            .packages
            .into_iter()
            .map(|package| (package.key.name, package.key.version))
            .collect(),
    })
}
/// Dev entries require a valid lock even when the manifest has no direct
/// dependencies. Test pre-flight keeps its documented lockless fixture
/// exception in [`check_any_freshness`].
/// Dev luôn cần lock hợp lệ, kể cả khi manifest rỗng dependency.
/// Return direct root names for one owner/ecosystem slice without confusing
/// transitive packages with dependencies declared by the project.
/// Trả tên root trực tiếp theo owner/ecosystem, không nhầm package bắc cầu
/// thành dependency do project khai báo.
pub(in crate::commands::freshness) fn locked_root_versions(
    project_root: &Path,
    owner: &str,
    ecosystem: mgc_lockfile::EcosystemTag,
) -> Result<BTreeMap<String, Vec<String>>> {
    use mgc_lockfile::{EcosystemTag, LockDocument, parse_root_pin};

    let lock_path = project_root.join("mgc.lock");
    let document = mgc_lockfile::parser::load_lock_document(&lock_path).map_err(|e| {
        crate::error::stale_lock_needs_install(&format!("cannot read mgc.lock: {e}"))
    })?;

    let mut versions = BTreeMap::<String, Vec<String>>::new();
    match document {
        LockDocument::Legacy(lockfile) => {
            let owner_roots: Vec<&String> = lockfile
                .root_dependencies_by_owner
                .get(owner)
                .into_iter()
                .flatten()
                .collect();
            let legacy_web_roots = owner == "web"
                && lockfile.version == "2"
                && ecosystem == EcosystemTag::Web
                && owner_roots.is_empty();
            let roots: Vec<&String> = if legacy_web_roots {
                lockfile.root_dependencies.iter().collect()
            } else {
                owner_roots
            };

            for root in roots {
                let parsed = parse_root_pin(root);
                if parsed.ecosystem.is_some_and(|tag| tag != ecosystem) {
                    continue;
                }
                let Some((name, version)) = parsed.package_id.rsplit_once('@') else {
                    return Err(crate::error::stale_lock_needs_install(&format!(
                        "invalid direct root pin '{root}'"
                    )));
                };
                if name.is_empty() || version.is_empty() {
                    return Err(crate::error::stale_lock_needs_install(&format!(
                        "invalid direct root pin '{root}'"
                    )));
                }
                let candidates = lockfile
                    .packages
                    .iter()
                    .filter(|package| package.name == name && package.version == version)
                    .filter(|package| {
                        package.ecosystem == ecosystem
                            || (legacy_web_roots
                                && ecosystem == EcosystemTag::Web
                                && package.ecosystem == EcosystemTag::Other)
                    })
                    .filter(|package| {
                        package
                            .owner_core
                            .as_deref()
                            .is_none_or(|value| value == owner)
                    })
                    .count();
                if candidates != 1 {
                    let reason = if candidates == 0 {
                        "does not identify a locked package"
                    } else {
                        "identifies multiple locked packages"
                    };
                    return Err(crate::error::stale_lock_needs_install(&format!(
                        "direct root pin '{root}' {reason}"
                    )));
                }
                versions
                    .entry(name.to_string())
                    .or_default()
                    .push(version.to_string());
            }
        }
        LockDocument::V4(lockfile) => {
            let roots = lockfile
                .root_dependencies_by_owner
                .get(owner)
                .into_iter()
                .flatten();
            for root in roots {
                let parsed = parse_root_pin(root);
                if parsed.ecosystem.is_some_and(|tag| tag != ecosystem) {
                    continue;
                }
                let Some((name, version)) = parsed.package_id.rsplit_once('@') else {
                    return Err(crate::error::stale_lock_needs_install(&format!(
                        "invalid direct root pin '{root}'"
                    )));
                };
                if name.is_empty() || version.is_empty() {
                    return Err(crate::error::stale_lock_needs_install(&format!(
                        "invalid direct root pin '{root}'"
                    )));
                }
                let candidates = lockfile
                    .packages
                    .iter()
                    .filter(|package| {
                        package.key.name == name
                            && package.key.version == version
                            && package.key.ecosystem == ecosystem
                    })
                    .count();
                if candidates != 1 {
                    let reason = if candidates == 0 {
                        "does not identify a locked package"
                    } else {
                        "identifies multiple locked packages"
                    };
                    return Err(crate::error::stale_lock_needs_install(&format!(
                        "direct root pin '{root}' {reason}"
                    )));
                }
                versions
                    .entry(name.to_string())
                    .or_default()
                    .push(version.to_string());
            }
        }
    }
    for package_versions in versions.values_mut() {
        package_versions.sort();
        package_versions.dedup();
    }
    Ok(versions)
}
/// Require declared dependency names and direct lock roots to be identical.
/// Yêu cầu tập tên khai báo trùng khớp root trực tiếp trong lock.
pub(in crate::commands::freshness) fn check_declared_roots(
    declared: &[String],
    locked_roots: &[String],
) -> Result<()> {
    let declared: BTreeSet<&str> = declared.iter().map(String::as_str).collect();
    let locked: BTreeSet<&str> = locked_roots.iter().map(String::as_str).collect();
    let missing: Vec<&str> = declared.difference(&locked).copied().collect();
    let stale: Vec<&str> = locked.difference(&declared).copied().collect();
    if !missing.is_empty() || !stale.is_empty() {
        let mut differences = Vec::new();
        if !missing.is_empty() {
            differences.push(format!("not root-locked: {}", missing.join(", ")));
        }
        if !stale.is_empty() {
            differences.push(format!("no longer declared: {}", stale.join(", ")));
        }
        return Err(crate::error::stale_lock_needs_install(
            &differences.join("; "),
        ));
    }
    Ok(())
}
pub(in crate::commands::freshness) fn check_manifest_roots(
    project_root: &Path,
    owner_fallback: &str,
    ecosystem: mgc_lockfile::EcosystemTag,
    declared: &[String],
) -> Result<()> {
    let owner = crate::commands::core::shared::lock_owner_core(project_root, owner_fallback)?;
    let locked = locked_root_versions(project_root, &owner, ecosystem)?;
    let locked_names = locked.keys().cloned().collect::<Vec<_>>();
    check_declared_roots(declared, &locked_names)
}
