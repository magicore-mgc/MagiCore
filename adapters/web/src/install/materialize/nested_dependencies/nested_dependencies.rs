//! Nested dependencies operations for Web dependency materialization.
//! Thao tác nested dependencies thuộc materialize dependency Web.

use std::path::Path;
use std::path::PathBuf;

use mgc_store::{ContentStore, Layout, PackageCache};
use mgc_types::adapter::ResolvedPackage;
use mgc_types::{MgError, MgResult, PackageId};

use crate::cache::SharedWebCache;
use crate::install::link_tree::hardlink_tree;
use crate::install::materialize::extracted_root_for;
use crate::install::script_policy::LifecyclePackage;
use crate::lockfile::installed_package_matches;

#[allow(clippy::too_many_arguments)]
pub fn materialize_nested_dependencies(
    package_dir: &Path,
    pkg: &ResolvedPackage,
    package_map: &std::collections::HashMap<PackageId, &ResolvedPackage>,
    root_package_versions: &std::collections::HashMap<String, PackageId>,
    layout: &Layout,
    store: &ContentStore,
    shared_cache: Option<&SharedWebCache>,
    cache: &PackageCache,
    extracted_roots: &mut std::collections::HashMap<PackageId, PathBuf>,
    visiting: &mut std::collections::HashSet<String>,
    depth: usize,
    packages_with_scripts: &mut Vec<LifecyclePackage>,
    generation: i64,
) -> MgResult<()> {
    const MAX_DEPTH: usize = 50;
    if depth > MAX_DEPTH {
        return Err(MgError::Other(format!(
            "dependency graph too deep (>{}) for '{}'",
            MAX_DEPTH,
            pkg.id.name_str()
        )));
    }
    if !visiting.insert(pkg.id.name_str().to_string()) {
        return Ok(());
    }

    let nested_node_modules = package_dir.join("node_modules");
    std::fs::create_dir_all(&nested_node_modules).map_err(|err| {
        MgError::Other(format!(
            "failed to create nested node_modules '{}' for '{}': {}",
            nested_node_modules.display(),
            pkg.id.name_str(),
            err
        ))
    })?;

    for dep_id in &pkg.deps {
        let Some(dep_pkg) = package_map.get(dep_id) else {
            continue;
        };
        let is_hoisted_match = root_package_versions
            .get(dep_id.name_str())
            .map(|root_id| root_id == dep_id)
            .unwrap_or(false);
        if is_hoisted_match {
            continue;
        }

        let nested_dir = nested_node_modules.join(dep_id.name().as_str());
        if !installed_package_matches(&nested_dir, dep_id) {
            if nested_dir.exists() {
                std::fs::remove_dir_all(&nested_dir).map_err(|err| {
                    MgError::Other(format!(
                        "failed to remove stale nested dependency '{}' for '{}': {}",
                        nested_dir.display(),
                        dep_id,
                        err
                    ))
                })?;
            }

            let package_root = extracted_root_for(
                extracted_roots,
                layout,
                store,
                shared_cache,
                cache,
                dep_pkg,
                generation,
            )?;
            hardlink_tree(package_root.as_path(), &nested_dir)?;
            packages_with_scripts.push(LifecyclePackage::new(dep_id.clone(), nested_dir.clone()));
        }

        materialize_nested_dependencies(
            &nested_dir,
            dep_pkg,
            package_map,
            root_package_versions,
            layout,
            store,
            shared_cache,
            cache,
            extracted_roots,
            visiting,
            depth + 1,
            packages_with_scripts,
            generation,
        )?;
    }

    visiting.remove(pkg.id.name_str());
    Ok(())
}

#[cfg(test)]
#[path = "test/nested_dependencies.rs"]
mod tests;
