//! Package-root location and cache-backed extraction.
//! Tìm package root và giải nén từ cache.
use super::super::{
    claims::{cas_claim_context, claim_ctx},
    materialize::ensure_extracted_package_root_with_marker,
    warm_cache::try_reuse_verified_warm_root,
};
use crate::install::package_marker::expected_extracted_package_marker_fast;
use crate::install::package_marker::expected_extracted_package_marker_from_bytes;
use mgc_fetcher::extract::{
    extract_tarball_to_rebuildable_cas_and_link,
    extract_tarball_to_rebuildable_cas_and_verify_existing_root,
};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use mgc_store::{ContentStore, Layout};
use mgc_types::adapter::ResolvedPackage;
use mgc_types::{MgError, MgResult};

use crate::cache::SharedWebCache;
use crate::install::package_marker::{
    expected_extracted_package_marker_fast_from_reader,
    expected_extracted_package_marker_from_reader,
};

#[path = "locate/mod.rs"]
mod locate;
#[path = "opening/mod.rs"]
mod opening;

pub use locate::locate_package_dir;
use opening::{open_rewound_cache_file, open_tarball_path};

pub fn ensure_extracted_package_root(
    layout: &Layout,
    store: &ContentStore,
    shared_cache: Option<&SharedWebCache>,
    pkg: &ResolvedPackage,
    tarball_path: &Path,
    generation: i64,
) -> MgResult<PathBuf> {
    let file = open_tarball_path(pkg, tarball_path)?;
    ensure_extracted_package_root_from_file(layout, store, shared_cache, pkg, file, generation)
}

pub fn ensure_extracted_package_root_from_file(
    layout: &Layout,
    store: &ContentStore,
    shared_cache: Option<&SharedWebCache>,
    pkg: &ResolvedPackage,
    tarball_file: File,
    generation: i64,
) -> MgResult<PathBuf> {
    let source = Arc::new(Mutex::new(tarball_file));
    let fast_marker = {
        let mut file = source
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        expected_extracted_package_marker_fast_from_reader(pkg, &mut *file)?
    };
    let expected_source = Arc::clone(&source);
    let expected_marker = move || {
        let mut file = expected_source
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        expected_extracted_package_marker_from_reader(pkg, &mut *file)
    };
    let open_source = Arc::clone(&source);
    let open_tarball = move || open_rewound_cache_file(&open_source);

    ensure_extracted_package_root_with_source(
        layout,
        store,
        shared_cache,
        pkg,
        PackageRootSource {
            fast_marker,
            expected_marker,
            open_tarball,
        },
        generation,
    )
}

struct PackageRootSource<E, O> {
    fast_marker: crate::cache::ExtractedPackageMarker,
    expected_marker: E,
    open_tarball: O,
}

fn ensure_extracted_package_root_with_source<E, O>(
    layout: &Layout,
    store: &ContentStore,
    shared_cache: Option<&SharedWebCache>,
    pkg: &ResolvedPackage,
    source: PackageRootSource<E, O>,
    generation: i64,
) -> MgResult<PathBuf>
where
    E: FnOnce() -> MgResult<crate::cache::ExtractedPackageMarker>,
    O: Fn() -> MgResult<File>,
{
    let PackageRootSource {
        fast_marker,
        expected_marker,
        open_tarball,
    } = source;
    let claim = cas_claim_context(layout, generation)?;
    if let Some(root) = try_reuse_verified_warm_root(
        layout,
        store,
        shared_cache,
        pkg,
        &fast_marker,
        &open_tarball,
        &claim,
    )? {
        return Ok(root);
    }

    let expected_marker = expected_marker()?;
    ensure_extracted_package_root_with_marker(
        layout,
        store,
        shared_cache,
        pkg,
        &expected_marker,
        |temp_root| {
            let file = open_tarball()?;
            extract_tarball_to_rebuildable_cas_and_link(
                file,
                temp_root,
                store,
                Some(claim_ctx(&claim)),
            )
            .map_err(|e| MgError::Other(e.to_string()))
        },
        |cached_root| {
            let file = open_tarball()?;
            extract_tarball_to_rebuildable_cas_and_verify_existing_root(
                file,
                cached_root,
                store,
                Some(claim_ctx(&claim)),
            )
            .map_err(|e| MgError::Other(e.to_string()))
        },
    )
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

pub fn ensure_extracted_package_root_from_bytes(
    layout: &Layout,
    store: &ContentStore,
    shared_cache: Option<&SharedWebCache>,
    pkg: &ResolvedPackage,
    tarball_bytes: &[u8],
    generation: i64,
) -> MgResult<PathBuf> {
    let fast_marker = expected_extracted_package_marker_fast(pkg, tarball_bytes);
    let claim = cas_claim_context(layout, generation)?;
    if let Some(root) = try_reuse_verified_warm_root(
        layout,
        store,
        shared_cache,
        pkg,
        &fast_marker,
        || Ok(std::io::Cursor::new(tarball_bytes)),
        &claim,
    )? {
        return Ok(root);
    }

    let expected_marker = expected_extracted_package_marker_from_bytes(pkg, tarball_bytes)?;
    ensure_extracted_package_root_with_marker(
        layout,
        store,
        shared_cache,
        pkg,
        &expected_marker,
        |temp_root| {
            extract_tarball_to_rebuildable_cas_and_link(
                std::io::Cursor::new(tarball_bytes),
                temp_root,
                store,
                Some(claim_ctx(&claim)),
            )
            .map_err(|e| MgError::Other(e.to_string()))
        },
        |cached_root| {
            extract_tarball_to_rebuildable_cas_and_verify_existing_root(
                std::io::Cursor::new(tarball_bytes),
                cached_root,
                store,
                Some(claim_ctx(&claim)),
            )
            .map_err(|e| MgError::Other(e.to_string()))
        },
    )
}
