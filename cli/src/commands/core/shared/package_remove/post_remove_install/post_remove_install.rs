//! Handle no-op removal, optional reinstall, and journal finish or rollback.
//! Xử lý remove không đổi, reinstall tùy chọn và finish/rollback journal.

use super::super::*;

/// Preserve the no-removal messages and return without touching the journal.
/// Giữ output khi không gỡ được package và return trước khi đụng journal.
pub(crate) fn finish_no_removal(install: bool) -> Result<()> {
    info("Manifest unchanged.");
    if install {
        info("Skipping reinstall because no dependencies were removed.");
    }
    Ok(())
}

/// Preserve install-disabled and native-install recovery behavior.
/// Giữ nhánh hoãn install và hành vi recovery của native install.
pub(crate) async fn finish_remove(
    adapter: &dyn PackageAdapter,
    root: &Path,
    install: bool,
    snapshot: &MutationSnapshot,
    write_lock: &ProjectWriteLock,
) -> Result<()> {
    if !install {
        info(&format!(
            "Run '{}' to update lockfile and node_modules",
            style_cmd(install_command_for_adapter(adapter))
        ));
        if let Err(error) = finish_mutation_journal(root, write_lock) {
            return rollback_mutation(adapter, root, snapshot, error, write_lock).await;
        }
        return Ok(());
    }
    info("Re-installing dependency graph...");
    // Test-only delay hook — lets E2E kill the process during the install tail.
    // (Hook chỉ dành cho test — cho phép E2E kill process giữa install tail.)
    if let Ok(ms) = std::env::var("MGC_MUTATION_TAIL_DELAY_MS")
        && let Ok(ms) = ms.parse::<u64>()
    {
        std::thread::sleep(std::time::Duration::from_millis(ms));
    }
    // Keep the optional pruned-graph lane disabled pending lockfile v2 migration.
    // (Giữ lane graph đã prune tắt cho đến khi migration lockfile v2 hoàn tất.)
    let graph = None;
    if let Some(graph) = graph {
        info("Using mgc.lock for remaining dependency graph.");
        let started_at = std::time::Instant::now();
        let spinner = create_spinner("  Linking packages...");
        let install_result = adapter
            .install(
                &graph,
                root,
                InstallOptions {
                    incremental: true,
                    ..Default::default()
                },
            )
            .await;
        spinner.finish_and_clear();
        let mut summary = match install_result {
            Ok(summary) => summary,
            Err(error) => {
                return rollback_mutation(adapter, root, snapshot, error, write_lock).await;
            }
        };
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
        if let Err(error) = finish_mutation_journal(root, write_lock) {
            return rollback_mutation(adapter, root, snapshot, error, write_lock).await;
        }
        return Ok(());
    }
    match install_with_adapter_locked(
        adapter,
        root,
        install_command_for_adapter(adapter),
        false,
        mgc_types::adapter::InstallOptions {
            incremental: true,
            ..Default::default()
        },
        write_lock,
    )
    .await
    {
        Ok(()) => {
            if let Err(error) = finish_mutation_journal(root, write_lock) {
                return rollback_mutation(adapter, root, snapshot, error, write_lock).await;
            }
            Ok(())
        }
        Err(error) => rollback_mutation(adapter, root, snapshot, error, write_lock).await,
    }
}
