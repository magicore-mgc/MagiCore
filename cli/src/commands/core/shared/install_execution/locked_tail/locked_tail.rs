//! Run install preparation and adapter materialization while the writer lock is held.
//! Chạy chuẩn bị install và materialize adapter khi writer lock còn được giữ.

use super::*;

pub(in crate::commands::core::shared::implementation::implementation) async fn install_with_adapter_locked(
    adapter: &dyn PackageAdapter,
    root: &Path,
    add_cmd: &str,
    frozen: bool,
    opts: mgc_types::adapter::InstallOptions,
    // Held alive by the caller across the whole tail (the borrow keeps
    // the guard — and the OS lock — from releasing mid-flight).
    // (Giữ guard sống xuyên tail.)
    _write_lock: &ProjectWriteLock,
) -> Result<()> {
    // NO recovery here by design: every caller either entered through
    // begin_dependency_mutation (recovery already ran) or IS the tail of
    // an op with a live staged journal — recovering now would "restore"
    // the outer op's own in-progress state mid-flight (the exact
    // self-recovery corruption). Stale journals are only ever handled at
    // gateway entries holding a fresh lock with nothing staged.
    // (Tail KHÔNG recovery — journal đang mở là của op ngoài.)
    let command_started_at = std::time::Instant::now();
    let InstallExecution {
        graph,
        summary: _prepared_summary,
        used_lockfile,
    } = prepare_install_execution(adapter, root, frozen || opts.offline, Some(add_cmd)).await?;
    profile_install_mark("prepare_install_execution", command_started_at);
    let started_at = std::time::Instant::now();

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
    let mut opts = opts;
    opts.replay_from_lock = used_lockfile;
    let mut summary = adapter.install(&graph, root, opts).await?;
    spinner.finish_and_clear();
    profile_install_mark("adapter_install", started_at);
    summary.duration_ms = started_at.elapsed().as_millis() as u64;

    // Cache-source label (B-series honesty): name WHERE the bytes
    // came from — the shared mgc store or the native toolchain cache.
    // Nhãn nguồn cache (B-series): nêu byte đến TỪ ĐÂU — store chung
    // của mgc hay cache toolchain gốc.
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
    profile_install_mark("install_with_adapter_total", command_started_at);
    Ok(())
}
