//! Match protocol packages to lock rows and validate resolved coverage.
//! Đối chiếu package theo protocol với lock và kiểm tra độ phủ graph.

use mgc_lockfile::EcosystemTag;
use mgc_types::{MgError, MgResult, ResolvedGraph};

use crate::language::LibLanguage;

/// Integrity markers for one graph package, reattached from its lock
/// entry by identity. Graph packages carry sha256/integrity only —
/// protocol-specific markers (go sumdb-ziphash, nuget sha512) travel in
/// the lock; without reattaching, the verifier sees neither source.
/// (Gắn lại marker toàn vẹn từ entry lock theo danh tính.)
pub(crate) fn markers_for(
    pkg: &mgc_types::ResolvedPackage,
    lock_packages: &[mgc_lockfile::Package],
) -> Vec<String> {
    lock_entry_for(pkg, lock_packages)
        .and_then(|p| p.markers.clone())
        .unwrap_or_default()
}

/// Lock entry for one graph package: exact name+version first, then
/// parse-equal fallback — Maven coordinates are NOT semver-normalizable
/// (`1.3` ≠ `1.3.0` on disk) while graph versions are normalized, so an
/// exact-only match orphans real entries.
/// (Tìm entry lock: khớp chính xác trước, rồi khớp nới lỏng.)
pub(crate) fn lock_entry_for<'a>(
    pkg: &mgc_types::ResolvedPackage,
    lock_packages: &'a [mgc_lockfile::Package],
) -> Option<&'a mgc_lockfile::Package> {
    let name = pkg.id.name_str();
    let version = pkg.id.version().to_string();
    if let Some(found) = lock_packages
        .iter()
        .find(|p| p.name == name && p.version == version)
    {
        return Some(found);
    }
    lock_packages.iter().find(|p| {
        p.name == name
            && mgc_types::Version::parse(&p.version).ok().as_ref() == Some(pkg.id.version())
    })
}

pub(crate) fn ecosystem_for_language(language: LibLanguage) -> EcosystemTag {
    match language {
        LibLanguage::Ts => EcosystemTag::Web,
        LibLanguage::Rust => EcosystemTag::Rust,
        LibLanguage::Python => EcosystemTag::Python,
        LibLanguage::Go => EcosystemTag::Go,
        LibLanguage::Java => EcosystemTag::Maven,
        LibLanguage::DotNet => EcosystemTag::NuGet,
    }
}

pub(crate) fn validate_lock_coverage(
    graph: &ResolvedGraph,
    ecosystem: EcosystemTag,
    lock_packages: &[mgc_lockfile::Package],
) -> MgResult<()> {
    if lock_packages
        .iter()
        .any(|package| package.ecosystem != ecosystem)
    {
        return Err(MgError::Other(format!(
            "resolver returned a lock entry outside active ecosystem '{ecosystem}'"
        )));
    }
    for resolved in &graph.packages {
        if lock_entry_for(resolved, lock_packages).is_none() {
            return Err(MgError::Other(format!(
                "refusing install: resolved package '{}'@{} has no matching integrity/lock entry",
                resolved.id.name_str(),
                resolved.id.version()
            )));
        }
    }
    Ok(())
}
