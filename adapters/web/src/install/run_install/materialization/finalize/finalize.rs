//! Final root pruning, staging cleanup, and binary-link rebuild.
//! Dọn root/staging lần cuối và dựng lại binary link.

use crate::install::bin::rebuild_bin_links;
use crate::install::run_install::state::InstallState;
use mgc_types::MgResult;
use mgc_types::adapter::ResolvedPackage;

pub(in crate::install::run_install::materialization) fn finish<'a>(
    state: &mut InstallState<'a>,
    mut affected_root_bin_links: Vec<&'a ResolvedPackage>,
) -> MgResult<()> {
    let project_root = state.project_root;
    let graph = state.graph;
    let node_modules = &state.node_modules;
    let root_packages = state.root_packages.clone();
    let root_package_versions = &state.root_package_versions;
    let staging_root = state.staging_root.as_ref();
    let shared_cache = state.shared_cache.as_ref();
    let profile = &mut state.profile;
    let start = state.start;

    if let Some(shared_cache) = shared_cache {
        let _ = shared_cache.write_project_ref(
            project_root,
            graph
                .packages
                .iter()
                .map(|pkg| shared_cache.extracted_package_root(pkg)),
        );
    }
    crate::install::materialize::prune_root_install_dirs(node_modules, root_package_versions)?;
    profile.mark("prune_root_install_dirs", start);
    if let Some(staging_root) = staging_root
        && staging_root.exists()
    {
        std::fs::remove_dir_all(staging_root).map_err(|err| {
            mgc_types::MgError::Other(format!(
                "failed to clean staging root '{}': {}",
                staging_root.display(),
                err
            ))
        })?;
    }
    if !node_modules.join(".bin").exists() && affected_root_bin_links.is_empty() {
        affected_root_bin_links = root_packages.to_vec();
    }
    crate::install::debug_trace::install_debug(format_args!("install:rebuild_bin_links_start"));
    rebuild_bin_links(
        node_modules,
        &root_packages,
        &affected_root_bin_links,
        !state.opts.legacy_flat,
    )?;
    profile.mark("rebuild_bin_links", start);
    Ok(())
}
