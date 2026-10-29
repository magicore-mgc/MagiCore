//! Materialize nested legacy dependencies or the strict virtual layout.
//! Materialize dependency lồng legacy hoặc virtual layout strict.

use crate::install::download::pipeline_download_and_extract;
use crate::install::materialize::{
    materialize_nested_dependencies, materialize_strict_layout, reset_nested_node_modules,
};
use crate::install::run_install::state::InstallState;
use crate::install::script_policy::LifecyclePackage;
use crate::lockfile::installed_package_matches;
use mgc_types::adapter::ResolvedPackage;
use mgc_types::{MgError, MgResult};

pub(in crate::install::run_install::materialization) async fn materialize_dependencies<'a>(
    state: &mut InstallState<'a>,
) -> MgResult<Vec<&'a ResolvedPackage>> {
    let opts = &state.opts;
    let fetch_graph = &state.fetch_graph;
    let start = state.start;
    let already_materialized = &state.already_materialized;
    let active_package_cache = if state.use_shared_primary {
        state
            .shared_package_cache_for_install
            .as_ref()
            .expect("shared package cache checked")
    } else {
        &state.cache
    };
    let registry = state.registry;
    let layout = &state.layout;
    let store = state.store_override.unwrap_or(&state.default_store);
    let shared_cache = &state.shared_cache;
    let cas_generation = state.cas_generation;
    let graph = state.graph;
    let package_map = &state.package_map;
    let root_packages = state.root_packages.clone();
    let node_modules = &state.node_modules;
    let root_package_versions = &state.root_package_versions;
    let packages_with_scripts = &mut state.packages_with_scripts;
    let summary = &mut state.summary;
    let profile = &mut state.profile;

    let mut affected_root_bin_links: Vec<&ResolvedPackage> = Vec::new();
    if opts.legacy_flat {
        affected_root_bin_links = root_packages.to_vec();
        for pkg in &root_packages {
            let package_dir = node_modules.join(pkg.id.name().as_str());
            reset_nested_node_modules(&package_dir)?;
            packages_with_scripts.push(LifecyclePackage::new(pkg.id.clone(), package_dir.clone()));
            let mut visiting = std::collections::HashSet::new();
            let mut extracted_roots = std::collections::HashMap::new();
            materialize_nested_dependencies(
                &package_dir,
                pkg,
                package_map,
                root_package_versions,
                layout,
                store,
                shared_cache.as_ref(),
                active_package_cache,
                &mut extracted_roots,
                &mut visiting,
                0,
                packages_with_scripts,
                cas_generation,
            )?;
        }
    } else if fetch_graph.is_empty() {
        profile.mark("prepare_extracted_roots", start);
    } else {
        let pipeline_step_started_at = std::time::Instant::now();
        eprintln!(
            "[magicore:debug] install:pipeline_start pkgs={} materialized={}",
            fetch_graph.packages.len(),
            already_materialized.len()
        );
        let (pipeline_bytes, extracted_roots, persist_handles) = pipeline_download_and_extract(
            fetch_graph,
            already_materialized,
            active_package_cache,
            shared_cache.as_ref(),
            Some(registry),
            layout,
            store,
            cas_generation,
        )
        .await?;
        eprintln!(
            "[magicore:debug] install:pipeline_done extracted={}",
            extracted_roots.len()
        );
        summary.bytes_from_cache += pipeline_bytes;
        profile.mark_step(
            "pipeline_download_and_extract_step",
            pipeline_step_started_at,
        );
        profile.mark("prepare_extracted_roots", start);
        let fetch_ids = fetch_graph
            .packages
            .iter()
            .map(|pkg| pkg.id.clone())
            .collect::<std::collections::HashSet<_>>();
        let root_packages_to_link = if opts.incremental {
            root_packages
                .iter()
                .copied()
                .filter(|pkg| {
                    fetch_ids.contains(&pkg.id)
                        || !installed_package_matches(
                            &node_modules.join(pkg.id.name().as_str()),
                            &pkg.id,
                        )
                })
                .collect::<Vec<_>>()
        } else {
            root_packages.to_vec()
        };
        affected_root_bin_links = root_packages_to_link.clone();

        let strict_materialize_step_started_at = std::time::Instant::now();
        eprintln!("[magicore:debug] install:strict_materialize_start");
        // Test-only failpoint: park just before the node_modules tree is
        // materialized (claims + CAS blobs are already committed upstream).
        // (Failpoint chỉ-cho-test: đỗ ngay trước khi cây node_modules được
        // materialize (claim + blob CAS đã commit ở upstream).)
        mgc_store::failpoint::hit("before-materialize");
        materialize_strict_layout(
            node_modules,
            graph,
            fetch_graph,
            package_map,
            &root_packages_to_link,
            layout,
            store,
            shared_cache.as_ref(),
            state.registry_url,
            active_package_cache,
            packages_with_scripts,
            &extracted_roots,
            cas_generation,
        )?;
        // Test-only failpoint: park after materialization committed the
        // hardlink tree (still before promote/refs).
        // (Failpoint chỉ-cho-test: đỗ sau khi materialization đã commit cây
        // hardlink (vẫn trước promote/refs).)
        mgc_store::failpoint::hit("after-materialize");
        eprintln!("[magicore:debug] install:strict_materialize_done");
        profile.mark_step(
            "materialize_strict_layout_step",
            strict_materialize_step_started_at,
        );
        let persist_step_started_at = std::time::Instant::now();
        for handle in persist_handles {
            handle
                .await
                .map_err(|e| MgError::Other(format!("shared cache persist task panicked: {e}")))?;
        }
        profile.mark_step("persist_shared_cache_step", persist_step_started_at);
    }
    Ok(affected_root_bin_links)
}
