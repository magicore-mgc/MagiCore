//! Package root operations for Web dependency materialization.
//! Thao tác package root thuộc materialize dependency Web.

use std::path::PathBuf;

use mgc_store::{ContentStore, Layout, PackageCache};
use mgc_types::adapter::ResolvedPackage;
use mgc_types::{MgError, MgResult, PackageId};

use crate::cache::SharedWebCache;
use crate::install::extract::ensure_extracted_package_root;

pub fn materialization_package_root(
    layout: &Layout,
    store: &ContentStore,
    shared_cache: Option<&SharedWebCache>,
    cache: &PackageCache,
    extracted_roots: &std::collections::HashMap<PackageId, PathBuf>,
    pkg: &ResolvedPackage,
    generation: i64,
) -> MgResult<PathBuf> {
    // let-chain edition 2024 — gộp điều kiện theo clippy 1.98.
    if let Some(root) = extracted_roots.get(&pkg.id)
        && root.join("package.json").exists()
    {
        return Ok(root.clone());
    }

    let local_tarball = cache.tarball_path(&pkg.id);
    if local_tarball.exists() {
        return ensure_extracted_package_root(
            layout,
            store,
            shared_cache,
            pkg,
            &local_tarball,
            generation,
        );
    }

    if let Some(shared_cache) = shared_cache {
        let shared_package_cache = shared_cache
            .package_cache()
            .map_err(|err| MgError::Store(err.to_string()))?;
        let shared_tarball = shared_package_cache.tarball_path(&pkg.id);
        if shared_tarball.exists() {
            return ensure_extracted_package_root(
                layout,
                store,
                Some(shared_cache),
                pkg,
                &shared_tarball,
                generation,
            );
        }
    }

    Err(MgError::Other(format!(
        "missing extracted root and cached tarball for '{}'",
        pkg.id
    )))
}

#[cfg(test)]
#[path = "test/package_root.rs"]
mod tests;
