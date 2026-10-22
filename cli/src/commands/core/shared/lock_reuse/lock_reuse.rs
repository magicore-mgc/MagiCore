//! Reuse a compatible prior lock graph for a dependency add.
//! Tái sử dụng lock graph cũ tương thích khi thêm dependency.

use super::*;

pub(in crate::commands::core::shared::implementation::implementation) async fn try_install_added_packages_from_lock(
    adapter: &dyn PackageAdapter,
    root: &Path,
    manifest_before_add: Option<&Manifest>,
    added_packages: &[AddedPackage],
) -> Result<bool> {
    if added_packages.is_empty() {
        return Ok(false);
    }
    let Some(previous_manifest) = manifest_before_add else {
        return Ok(false);
    };
    // Delta install would retain old pins without registry publication data;
    // an active age gate must rebuild the complete graph from fresh metadata.
    // (Delta giữ pin cũ thiếu timestamp; age gate phải resolve lại toàn graph.)
    if refresh_locked_graph_for_age_gate(adapter, root, true, false)? {
        return Ok(false);
    }
    let Some(locked_graph) = load_locked_graph(root, adapter.name(), previous_manifest)? else {
        return Ok(false);
    };

    let delta_manifest = build_delta_manifest(previous_manifest, added_packages)?;

    let resolve_started_at = std::time::Instant::now();
    let spinner = create_spinner(&format!(
        "  Resolving {} new package(s)...",
        added_packages.len()
    ));
    // P0/F6: same per-operation arming for the delta resolve.
    adapter.arm_age_gate_for(root)?;
    let delta_graph = adapter.resolve(&delta_manifest).await?;
    spinner.finish_and_clear();
    profile_install_mark("resolve_delta_graph", resolve_started_at);

    let graph = merge_graphs(locked_graph, delta_graph);
    let added_ids = added_packages
        .iter()
        .map(|pkg| pkg.id.clone())
        .collect::<Vec<_>>();
    let started_at = std::time::Instant::now();
    let spinner = create_spinner("  Linking changed packages...");
    let mut summary = adapter
        .install(
            &graph,
            root,
            InstallOptions {
                incremental: true,
                force_install: added_ids.to_vec(),
                ..Default::default()
            },
        )
        .await?;
    spinner.finish_and_clear();
    summary.duration_ms = started_at.elapsed().as_millis() as u64;
    // Cache-source label (B-series honesty) — see install_with_adapter.
    // Nhãn nguồn cache (B-series) — xem install_with_adapter.
    let cache_source = match summary.cache_mode {
        mgc_types::adapter::InstallCacheMode::MgCStore => "shared mgc store",
        mgc_types::adapter::InstallCacheMode::Delegated => "native toolchain cache",
    };
    mgc_ui::print_install_summary_source(
        summary.added.len(),
        summary.bytes_from_cache as usize,
        summary.duration_ms,
        "0 B",
        Some(cache_source),
    );
    mgc_ui::blank_line();
    success("All dependencies installed");
    profile_install_mark("install_delta_with_lock_total", started_at);
    Ok(true)
}
