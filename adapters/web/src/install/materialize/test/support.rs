#![allow(clippy::unwrap_used)]
// Shared materialization fixtures — fixture chung cho materialization.
// These helpers create canonical temporary paths and typed package/store inputs.
// Helper dựng path tạm canonical cùng dữ liệu package/store có kiểu.
use std::path::Path;

use mgc_store::{ContentStore, Layout, PackageCache};
use mgc_types::PackageId;
use mgc_types::adapter::ResolvedPackage;

pub(crate) fn tempdir_real() -> tempfile::TempDir {
    let base = std::env::temp_dir().canonicalize().unwrap();
    tempfile::tempdir_in(base).unwrap()
}

pub(crate) fn package_id(spec: &str) -> PackageId {
    PackageId::parse(spec).unwrap()
}

pub(crate) fn resolved_package(
    id: PackageId,
    direct: bool,
    deps: Vec<PackageId>,
) -> ResolvedPackage {
    ResolvedPackage {
        id,
        integrity: String::new(),
        tarball_url: String::new(),
        deps,
        peer_deps: Vec::new(),
        direct,
        dev: false,
    }
}

pub(crate) fn storage_handles(root: &Path) -> (Layout, ContentStore, PackageCache) {
    let layout = Layout::new(root.join(".magicore/store"));
    let store = ContentStore::new(layout.cas_dir()).unwrap();
    let cache = PackageCache::new(layout.cache_dir()).unwrap();
    (layout, store, cache)
}
