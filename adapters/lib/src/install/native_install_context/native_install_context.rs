//! Build protocol entries and the shared content store for native lanes.
//! Dựng protocol entry và content store dùng chung cho lane native.

use mgc_resolver::protocols::ResolvedEntry;
use mgc_store::ContentStore;
use mgc_types::{MgError, MgResult, ResolvedPackage};

/// Reconstruct a protocol `ResolvedEntry` from a resolved graph package.
/// Dựng lại `ResolvedEntry` của protocol từ một package trong graph đã resolve.
pub fn entry_from_package(pkg: &ResolvedPackage) -> ResolvedEntry {
    ResolvedEntry {
        name: pkg.id.name_str().to_string(),
        version: pkg.id.version().to_string(),
        deps: Vec::new(),
        artifact_url: pkg.tarball_url.clone(),
        sha256: mgc_lockfile::policy::native_sha256_hex(&pkg.integrity),
        extra_markers: Vec::new(),
    }
}

/// Build (and create) the mgc CAS content store used by native install.
/// Dựng (và tạo) content store CAS mgc cho install native.
pub(crate) fn content_store() -> MgResult<ContentStore> {
    ContentStore::new(mgc_store::default_store_root()).map_err(|e| MgError::Store(e.to_string()))
}
