use anyhow::Result;
use mgc_types::Manifest;
use mgc_ui::{
    add_multi_bar, create_multi_progress, create_progress_bar, create_spinner, info, success,
};
use std::path::Path;

use super::super::locked_graph::load_locked_graph;

/// Generic-path resolve + materialize + hooks tail (P0-1): runs under
/// the caller's writer guard; every error after the manifest commit
/// point routes through rollback there. Test-only tail delay hook
/// (MGC_MUTATION_TAIL_DELAY_MS) parks SIGKILL E2E mid-tail.
/// (Tail resolve+install+hooks của generic path — lỗi là rollback.)
#[allow(clippy::too_many_arguments)]
pub(in crate::commands::install) async fn install_resolve_and_materialize(
    adapter: &dyn mgc_types::adapter::PackageAdapter,
    project_root: &Path,
    manifest: &Manifest,
    frozen: bool,
    ignore_scripts: bool,
    allow_scripts: bool,
    offline: bool,
) -> Result<()> {
    if let Ok(ms) = std::env::var("MGC_MUTATION_TAIL_DELAY_MS")
        && let Ok(ms) = ms.parse::<u64>()
    {
        std::thread::sleep(std::time::Duration::from_millis(ms));
    }
    let started_at = std::time::Instant::now();
    // SDK registry requirements must match the native resolver's effective manifest.
    // So lock bằng dependency hiệu lực của SDK, không ghi chúng vào manifest project.
    let replay_manifest = adapter.lock_replay_manifest(project_root, manifest)?;
    let locked_graph = load_locked_graph(project_root, adapter.name(), &replay_manifest)?;
    let force_fresh_resolve = crate::commands::core::shared::refresh_locked_graph_for_age_gate(
        adapter,
        project_root,
        locked_graph.is_some(),
        frozen,
    )?;
    let locked_graph = if force_fresh_resolve {
        None
    } else {
        locked_graph
    };
    let (graph, used_lockfile) = if let Some(graph) = locked_graph {
        info("Using mgc.lock for install state.");
        (graph, true)
    } else {
        // Frozen mode fails LOUDLY on mismatch (mirrors core/shared.rs):
        // silently re-resolving would bless a tampered lock and rewrite
        // it. Missing lock vs mismatched lock get distinct errors.
        if frozen {
            if project_root.join("mgc.lock").is_file() {
                return Err(crate::error::frozen_lock_mismatch("install"));
            }
            return Err(crate::error::frozen_lock_missing("install"));
        }
        let dep_count = manifest.all_dependencies().count();
        let spinner = create_spinner(&format!("  Resolving {dep_count} dependencies..."));
        // P0/F6: arm the age gate from THIS operation's project.
        adapter.arm_age_gate_for(project_root)?;
        let graph = adapter.resolve(manifest).await?;
        spinner.finish_and_clear();
        (graph, false)
    };

    let resolve_bar = create_progress_bar(graph.len() as u64, "Resolving...");
    if used_lockfile {
        resolve_bar.finish_with_message(format!(" Loaded {} locked packages", graph.len()));
    } else {
        resolve_bar.finish_with_message(format!(" Resolved {} packages", graph.len()));
    }

    let multi = create_multi_progress();
    let mut bars = vec![];
    for pkg in &graph.packages {
        let pb = add_multi_bar(
            &multi,
            100,
            &format!("{}@{}", pkg.id.name_str(), pkg.id.version()),
        );
        bars.push(pb);
    }

    for (i, pb) in bars.iter().enumerate() {
        pb.set_position(100);
        pb.finish_with_message(graph.packages[i].id.name_str().to_string());
    }

    let spinner = create_spinner("  Linking packages...");

    let opts = mgc_types::adapter::InstallOptions {
        ignore_scripts,
        allow_scripts,
        legacy_flat: crate::commands::core::shared::should_use_legacy_flat_layout(adapter.name()),
        frozen,
        offline,
        replay_from_lock: used_lockfile,
        ..Default::default()
    };
    let mut summary = adapter.install(&graph, project_root, opts).await?;
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

    // Post-install hooks (mgc.hooks.toml [hooks.post-install]) — fail → error
    // Post-install chạy sau khi node_modules đã materialize xong.
    crate::commands::hooks::run_event(project_root, "post-install")?;

    Ok(())
}
