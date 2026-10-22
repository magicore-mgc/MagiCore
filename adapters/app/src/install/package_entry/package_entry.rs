//! Convert resolved package records into protocol entries shared by App lanes.
//! Chuyển package đã resolve thành protocol entry dùng chung giữa các lane App.
use mgc_resolver::protocols::ResolvedEntry;
use mgc_types::ResolvedPackage;

/// Reconstruct a protocol `ResolvedEntry` from a resolved graph package.
/// Dựng lại `ResolvedEntry` của protocol từ một package trong graph đã resolve.
pub(crate) fn entry_from_package(pkg: &ResolvedPackage) -> ResolvedEntry {
    ResolvedEntry {
        name: pkg.id.name_str().to_string(),
        version: pkg.id.version().to_string(),
        deps: Vec::new(),
        artifact_url: pkg.tarball_url.clone(),
        sha256: mgc_lockfile::policy::native_sha256_hex(&pkg.integrity),
        extra_markers: Vec::new(),
    }
}
