//! Materialize operations for Web dependency materialization.
//! Thao tác materialize thuộc materialize dependency Web.

use std::path::{Path, PathBuf};

use mgc_store::{ContentStore, Layout, PackageCache};
use mgc_types::adapter::{ResolvedGraph, ResolvedPackage};
use mgc_types::{MgError, MgResult, PackageId};
use rayon::prelude::*;

use crate::cache::SharedWebCache;
use crate::install::extract::{
    materialized_package_matches, read_extracted_package_marker, write_materialized_package_marker,
};
use crate::install::link_tree::link_package_tree_with_profile;
use crate::install::materialize::{
    materialization_package_root, remove_fs_entry, strict_vstore_node_modules_dir,
    strict_vstore_package_dir,
};
use crate::install::script_policy::LifecyclePackage;
use crate::profile::MaterializationProfile;

#[allow(clippy::too_many_arguments)]
pub fn materialize_strict_layout(
    node_modules: &Path,
    graph: &ResolvedGraph,
    materialization_graph: &ResolvedGraph,
    package_map: &std::collections::HashMap<PackageId, &ResolvedPackage>,
    root_packages: &[&ResolvedPackage],
    layout: &Layout,
    store: &ContentStore,
    shared_cache: Option<&SharedWebCache>,
    cache: &PackageCache,
    packages_with_scripts: &mut Vec<LifecyclePackage>,
    extracted_roots: &std::collections::HashMap<PackageId, PathBuf>,
    generation: i64,
) -> MgResult<()> {
    let virtual_store = node_modules.join(".magicore");
    if let Err(e) = std::fs::create_dir_all(&virtual_store) {
        return Err(MgError::Other(format!(
            "failed to create virtual store: {}",
            e
        )));
    }
    let materialization_profile = MaterializationProfile::from_env();

    let vstore_dirs: Vec<_> = materialization_graph
        .packages
        .iter()
        .map(|pkg| {
            let pkg_id = &pkg.id;
            let vstore_pkg_dir = strict_vstore_package_dir(node_modules, pkg_id);
            (pkg_id.clone(), vstore_pkg_dir, pkg.clone())
        })
        .collect();
    let vstore_dir_map: std::collections::HashMap<PackageId, PathBuf> = graph
        .packages
        .iter()
        .map(|pkg| {
            (
                pkg.id.clone(),
                strict_vstore_package_dir(node_modules, &pkg.id),
            )
        })
        .collect();

    let materialize_results: Vec<_> = vstore_dirs
        .into_par_iter()
        .map(|(pkg_id, vstore_pkg_dir, pkg)| {
            let package_root = materialization_package_root(
                layout,
                store,
                shared_cache,
                cache,
                extracted_roots,
                &pkg,
                generation,
            )?;
            let source_marker = read_extracted_package_marker(&package_root)?;
            if !materialized_package_matches(&vstore_pkg_dir, &pkg_id, source_marker.as_ref())? {
                remove_fs_entry(&vstore_pkg_dir)?;
                if let Some(parent) = vstore_pkg_dir.parent() {
                    std::fs::create_dir_all(parent).map_err(|err| {
                        MgError::Other(format!(
                            "failed to create vstore parent '{}': {}",
                            parent.display(),
                            err
                        ))
                    })?;
                }
                link_package_tree_with_profile(
                    &package_root,
                    &vstore_pkg_dir,
                    Some(&materialization_profile),
                )?;
                write_materialized_package_marker(&vstore_pkg_dir, source_marker.as_ref())?;
            }
            Ok::<_, MgError>((vstore_pkg_dir, pkg_id))
        })
        .collect();

    for result in materialize_results {
        let (directory, package_id) = result?;
        packages_with_scripts.push(LifecyclePackage::new(package_id, directory));
    }

    let link_results: Vec<_> = materialization_graph
        .packages
        .par_iter()
        .map(|pkg| {
            let pkg_id = &pkg.id;
            if !vstore_dir_map.contains_key(pkg_id) {
                return Err(MgError::Other(format!(
                    "missing virtual store path for '{}'",
                    pkg_id
                )));
            }
            let vstore_node_modules = strict_vstore_node_modules_dir(node_modules, pkg_id);
            let pkg_local_node_modules = vstore_node_modules
                .join(pkg_id.name().as_str())
                .join("node_modules");

            if !pkg.deps.is_empty() {
                std::fs::create_dir_all(&pkg_local_node_modules).map_err(|err| {
                    MgError::Other(format!(
                        "failed to create strict nested node_modules '{}' for '{}': {}",
                        pkg_local_node_modules.display(),
                        pkg_id.name_str(),
                        err
                    ))
                })?;
            }

            for dep_id in &pkg.deps {
                if let Some(_dep_pkg) = package_map.get(dep_id) {
                    let Some(dep_vstore_pkg_dir) = vstore_dir_map.get(dep_id) else {
                        return Err(MgError::Other(format!(
                            "missing dependency virtual store path for '{}'",
                            dep_id
                        )));
                    };

                    let symlink_path = vstore_node_modules.join(dep_id.name().as_str());
                    crate::layout::create_symlink(dep_vstore_pkg_dir, &symlink_path)?;
                    materialization_profile.record_symlink();

                    let local_symlink_path = pkg_local_node_modules.join(dep_id.name().as_str());
                    crate::layout::create_symlink(dep_vstore_pkg_dir, &local_symlink_path)?;
                    materialization_profile.record_symlink();
                }
            }

            if !pkg.peer_deps.is_empty() {
                std::fs::create_dir_all(&pkg_local_node_modules).map_err(|err| {
                    MgError::Other(format!(
                        "failed to create strict nested node_modules '{}' for peer deps of '{}': {}",
                        pkg_local_node_modules.display(),
                        pkg_id.name_str(),
                        err
                    ))
                })?;
            }
            for peer_id in &pkg.peer_deps {
                if let Some(dep_vstore_pkg_dir) = vstore_dir_map.get(peer_id) {
                    let symlink_path = vstore_node_modules.join(peer_id.name().as_str());
                    crate::layout::create_symlink(dep_vstore_pkg_dir, &symlink_path)?;
                    materialization_profile.record_symlink();
                    let local_symlink_path = pkg_local_node_modules.join(peer_id.name().as_str());
                    crate::layout::create_symlink(dep_vstore_pkg_dir, &local_symlink_path)?;
                    materialization_profile.record_symlink();
                }
            }

            Ok::<_, MgError>(())
        })
        .collect();

    for result in link_results {
        result?;
    }

    for pkg in root_packages {
        let root_link = node_modules.join(pkg.id.name().as_str());
        let Some(vstore_pkg_dir) = vstore_dir_map.get(&pkg.id) else {
            return Err(MgError::Other(format!(
                "missing root virtual store path for '{}'",
                pkg.id
            )));
        };
        crate::layout::create_symlink(vstore_pkg_dir, &root_link)?;
        materialization_profile.record_symlink();
    }

    materialization_profile.flush();
    Ok(())
}

#[cfg(test)]
#[path = "test/materialize.rs"]
mod tests;
