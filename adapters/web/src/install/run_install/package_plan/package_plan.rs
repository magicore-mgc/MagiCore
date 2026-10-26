//! Package selection, cache choice, and offline preflight.
//! Chọn package, cache và preflight offline.

use crate::install::materialize::graph_without_packages;
use crate::install::run_install::state::InstallState;
use mgc_types::PackageId;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};

pub(in crate::install::run_install) fn prepare(
    state: &mut InstallState<'_>,
) -> mgc_types::MgResult<()> {
    state.root_package_versions = state
        .root_packages
        .iter()
        .map(|package| (package.id.name_str().to_string(), package.id.clone()))
        .collect::<HashMap<String, PackageId>>();
    state.package_map = state
        .graph
        .packages
        .iter()
        .map(|package| (package.id.clone(), package))
        .collect::<HashMap<PackageId, _>>();

    state.already_materialized = if state.opts.incremental {
        state
            .root_packages
            .par_iter()
            .filter(|package| {
                !state.opts.force_install.contains(&package.id)
                    && crate::lockfile::installed_package_matches(
                        &state.node_modules.join(package.id.name().as_str()),
                        &package.id,
                    )
            })
            .map(|package| package.id.clone())
            .collect()
    } else {
        state
            .root_packages
            .par_iter()
            .filter(|package| {
                crate::lockfile::installed_package_matches(
                    &state.node_modules.join(package.id.name().as_str()),
                    &package.id,
                )
            })
            .map(|package| package.id.clone())
            .collect()
    };
    // A package counts as present in the virtual store only when its marker
    // still matches the recorded content signature.
    //
    // Version equality alone is not enough: the no-op path already refuses a
    // tree whose bytes changed, so trusting the version here made the two paths
    // disagree — the no-op said "not intact", the plan said "already there",
    // `fetch_graph` came out empty, and nothing repaired the package. A tampered
    // file survived every later install.
    // Package chỉ được coi là có trong virtual store khi marker còn khớp chữ ký
    // nội dung đã ghi.
    //
    // Chỉ bằng version là chưa đủ: đường no-op đã từ chối cây có byte đổi, nên
    // tin version ở đây làm hai đường mâu thuẫn — no-op nói "không nguyên vẹn",
    // plan nói "đã có", `fetch_graph` rỗng, và không gì sửa package. File bị sửa
    // sống qua mọi lần install sau.
    let vstore_intact = |package: &mgc_types::adapter::ResolvedPackage| {
        let dir = crate::install::materialize::strict_vstore_package_dir(
            &state.node_modules,
            &package.id,
        );
        if !crate::lockfile::installed_package_matches(&dir, &package.id) {
            return false;
        }
        match crate::install::package_marker::read_extracted_package_marker(&dir) {
            // MagiCore always writes a marker into the virtual store it
            // produced, so a store it made is checked byte for byte.
            // MagiCore luôn ghi marker vào virtual store nó tạo, nên store do nó
            // tạo được kiểm từng byte.
            Ok(Some(marker)) => {
                crate::install::package_marker::extracted_content_matches(&dir, &marker)
                    .unwrap_or(false)
            }
            // No marker: the tree was laid down by another installer, and the
            // version contract is all there is to check. Reinstalling on sight
            // would refuse to work in a directory another tool owns.
            // Không có marker: cây do installer khác dựng, chỉ có hợp đồng
            // version để kiểm. Cài lại ngay sẽ từ chối làm việc trong thư mục do
            // tool khác sở hữu.
            _ => true,
        }
    };
    state.already_in_virtual_store = if state.opts.incremental {
        state
            .graph
            .packages
            .par_iter()
            .filter(|package| {
                !state.opts.force_install.contains(&package.id) && vstore_intact(package)
            })
            .map(|package| package.id.clone())
            .collect()
    } else {
        HashSet::new()
    };

    state.shared_package_cache_for_install = state
        .shared_cache
        .as_ref()
        .and_then(|shared| shared.package_cache().ok());
    let local_has_seeded_tarballs = state
        .graph
        .packages
        .iter()
        .any(|package| state.cache.contains_tarball(&package.id));
    state.use_shared_primary =
        !local_has_seeded_tarballs && state.shared_package_cache_for_install.is_some();

    state.fetch_graph = if state.opts.incremental && !state.already_in_virtual_store.is_empty() {
        if std::env::var("MAGICORE_WEB_PROFILE_INSTALL").is_ok() {
            eprintln!(
                "[magicore:web:materialize-profile] fetch_graph={} already_in_vstore={} graph_total={}",
                graph_without_packages(state.graph, &state.already_in_virtual_store)
                    .packages
                    .len(),
                state.already_in_virtual_store.len(),
                state.graph.packages.len()
            );
        }
        graph_without_packages(state.graph, &state.already_in_virtual_store)
    } else {
        state.graph.clone()
    };

    // Offline coverage is checked for the complete fetchable graph before any
    // download path; an incomplete cache never falls through to network.
    // Kiểm tra đủ cache cho toàn fetch graph trước khi đi vào nhánh tải mạng.
    if state.opts.offline && !state.fetch_graph.is_empty() {
        let missing: Vec<String> = state
            .fetch_graph
            .packages
            .iter()
            .filter(|package| {
                !state.cache.contains_tarball(&package.id)
                    && state
                        .shared_package_cache_for_install
                        .as_ref()
                        .map(|shared| !shared.contains_tarball(&package.id))
                        .unwrap_or(true)
            })
            .map(|package| package.id.to_string())
            .collect();
        if !missing.is_empty() {
            return Err(mgc_types::MgError::Other(format!(
                "offline install: {} package(s) not in local cache and cannot be fetched \
                 without network:\n  {}",
                missing.len(),
                missing.join("\n  ")
            )));
        }
    }
    Ok(())
}
