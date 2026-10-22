//! Root package extraction and promotion for legacy-flat installs.
//! Lấy package gốc và promote cho install legacy-flat.

use crate::install::materialize::{extracted_root_for, hardlink_tree};
use crate::install::run_install::state::InstallState;
use crate::lockfile::installed_package_matches;
use mgc_types::{MgError, MgResult};

pub(in crate::install::run_install::materialization) fn materialize_root_packages(
    state: &mut InstallState<'_>,
) -> MgResult<()> {
    let root_packages = state.root_packages.clone();
    let node_modules = state.node_modules.clone();
    let staging_root = &state.staging_root;
    let layout = &state.layout;
    let store = state.store_override.unwrap_or(&state.default_store);
    let shared_cache = &state.shared_cache;
    let active_package_cache = if state.use_shared_primary {
        state
            .shared_package_cache_for_install
            .as_ref()
            .expect("shared package cache checked")
    } else {
        &state.cache
    };
    let database = state.database.as_ref();
    let already_materialized = &state.already_materialized;
    let opts = &state.opts;
    let cas_generation = state.cas_generation;
    let summary = &mut state.summary;
    let profile = &mut state.profile;
    let start = state.start;

    let mut extracted_roots = std::collections::HashMap::new();
    profile.mark("prepare_extracted_roots", start);

    for pkg in &root_packages {
        let final_dir = node_modules.join(pkg.id.name().as_str());
        if installed_package_matches(&final_dir, &pkg.id) {
            if let Some(database) = database.as_ref() {
                database
                    .insert_package(
                        &pkg.id,
                        if pkg.integrity.is_empty() {
                            None
                        } else {
                            Some(pkg.integrity.as_str())
                        },
                    )
                    .map_err(|e| MgError::Store(e.to_string()))?;
            }
            if !opts.incremental || !already_materialized.contains(&pkg.id) {
                summary.added.push(pkg.id.clone());
            }
            continue;
        }

        let package_root = match extracted_root_for(
            &mut extracted_roots,
            layout,
            store,
            shared_cache.as_ref(),
            active_package_cache,
            pkg,
            cas_generation,
        ) {
            Ok(root) => root,
            Err(err) => {
                // let-chain edition 2024 — gộp điều kiện theo clippy 1.98.
                if let Some(staging_root) = staging_root.as_ref()
                    && staging_root.exists()
                {
                    let _ = std::fs::remove_dir_all(staging_root);
                }
                return Err(err);
            }
        };
        let materialized_dir = staging_root
            .as_ref()
            .expect("legacy-flat installs always create staging_root")
            .join("node_modules")
            .join(pkg.id.name().as_str());
        if materialized_dir.exists() {
            std::fs::remove_dir_all(&materialized_dir).map_err(|e| {
                MgError::Other(format!(
                    "failed to remove existing materialized dir '{}': {e}",
                    materialized_dir.display()
                ))
            })?;
        }
        // let-chain edition 2024 — gộp điều kiện theo clippy 1.98.
        if let Err(err) = hardlink_tree(package_root.as_path(), &materialized_dir) {
            if let Some(staging_root) = staging_root.as_ref()
                && staging_root.exists()
            {
                let _ = std::fs::remove_dir_all(staging_root);
            }
            return Err(err);
        }
        // let-chain edition 2024 — gộp điều kiện theo clippy 1.98.
        if let Some(database) = database.as_ref()
            && let Err(err) = database
                .insert_package(
                    &pkg.id,
                    if pkg.integrity.is_empty() {
                        None
                    } else {
                        Some(pkg.integrity.as_str())
                    },
                )
                .map_err(|e| MgError::Store(e.to_string()))
        {
            if let Some(staging_root) = staging_root.as_ref()
                && staging_root.exists()
            {
                let _ = std::fs::remove_dir_all(staging_root);
            }
            return Err(err);
        }
        if !opts.incremental || !already_materialized.contains(&pkg.id) {
            summary.added.push(pkg.id.clone());
        }
    }

    for pkg in &root_packages {
        let staged_dir = staging_root
            .as_ref()
            .expect("legacy-flat installs always create staging_root")
            .join("node_modules")
            .join(pkg.id.name().as_str());
        let final_dir = node_modules.join(pkg.id.name().as_str());
        if !staged_dir.exists() {
            continue;
        }
        if let Some(parent) = final_dir.parent() {
            std::fs::create_dir_all(parent).map_err(|err| {
                MgError::Other(format!(
                    "failed to create parent '{}' for '{}': {}",
                    parent.display(),
                    pkg.id.name_str(),
                    err
                ))
            })?;
        }
        if final_dir.exists() {
            std::fs::remove_dir_all(&final_dir).map_err(|err| {
                MgError::Other(format!(
                    "failed to remove existing install dir '{}' for '{}': {}",
                    final_dir.display(),
                    pkg.id.name_str(),
                    err
                ))
            })?;
        }
        std::fs::rename(&staged_dir, &final_dir).map_err(|err| {
            MgError::Other(format!(
                "failed to promote staged package '{}' from '{}' to '{}': {}",
                pkg.id.name_str(),
                staged_dir.display(),
                final_dir.display(),
                err
            ))
        })?;
    }
    Ok(())
}
