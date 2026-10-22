//! Extracted root operations for Web dependency materialization.
//! Thao tác extracted root thuộc materialize dependency Web.

use std::path::PathBuf;

use mgc_store::{ContentStore, Layout, PackageCache};
use mgc_types::adapter::ResolvedPackage;
use mgc_types::{MgResult, PackageId};

use crate::cache::SharedWebCache;
use crate::install::extract::ensure_extracted_package_root;

pub fn extracted_root_for(
    extracted_roots: &mut std::collections::HashMap<PackageId, PathBuf>,
    layout: &Layout,
    store: &ContentStore,
    shared_cache: Option<&SharedWebCache>,
    cache: &PackageCache,
    pkg: &ResolvedPackage,
    generation: i64,
) -> MgResult<PathBuf> {
    if let Some(existing) = extracted_roots.get(&pkg.id) {
        return Ok(existing.clone());
    }

    let tarball_path = cache.tarball_path(&pkg.id);
    let root =
        ensure_extracted_package_root(layout, store, shared_cache, pkg, &tarball_path, generation)?;
    extracted_roots.insert(pkg.id.clone(), root.clone());
    Ok(root)
}

#[cfg(test)]
#[path = "test/extracted_root.rs"]
mod tests;
