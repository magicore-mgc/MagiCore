//! Dedupe transaction coordinator — preserves lock, build, rollback and post-verify GC order.
//! Điều phối transaction dedupe — giữ thứ tự lock, build, rollback và GC sau xác minh.
use anyhow::{Context, Result, bail};
use mgc_config::project::ProjectConfig;

use super::build_verification::verify_with_build;
use super::lockfile_merge::merged_lockfile;
use super::report::print_report;
use super::types::{DedupeArgs, DedupeReport};
use super::virtual_store_cleanup::{GcReport, cleanup_unreferenced_vstore};

fn dry_run_report(before: usize, after: usize, merged: usize) -> DedupeReport {
    DedupeReport {
        before_instances: before,
        after_instances: after,
        merged,
        disk_saved_bytes: 0,
        entries: Vec::new(),
        gc_failed_paths: Vec::new(),
    }
}

pub async fn run(args: DedupeArgs) -> Result<()> {
    let cwd = std::env::current_dir()?;
    let project_root =
        ProjectConfig::find_project_root(&cwd).ok_or_else(crate::error::project_root_missing)?;

    let mgc_lock = project_root.join("mgc.lock");
    // Writer lock FIRST (P0-2/P0-3): every authoritative read below
    // happens inside the critical section — no stale-read race, no
    // interleave with concurrent mutations, and no pending journal is
    // paved over (fail closed instead).
    // (Lock trước, đọc sau — không stale-read, không đè journal.)
    let guard = mgc_lockfile::project_lock::ProjectWriteLock::acquire(
        &project_root,
        crate::commands::core::shared::writer_lock_timeout(&project_root),
    )
    .map_err(|e| anyhow::anyhow!("dedupe cannot acquire the project writer lock: {e}"))?;
    // Pending-journal protocol (P0-3): never rewrite the lock over an
    // unrestored mutation — fail closed and point at recovery. (Dedupe
    // has no adapter to restore with, so it refuses instead of paving.)
    // (Không đè lên journal chưa phục hồi — lỗi rõ.)
    crate::commands::core::shared::ensure_no_pending_remove_journal(&project_root, &guard)?;

    let lock_bytes = match mgc_lockfile::read_lockfile_bytes(&mgc_lock) {
        Ok(bytes) => bytes,
        Err(mgc_lockfile::LockfileError::IoError(error))
            if error.kind() == std::io::ErrorKind::NotFound =>
        {
            bail!("mgc.lock not found — run mgc install first");
        }
        Err(error) => return Err(error.into()),
    };
    let lock_content = String::from_utf8(lock_bytes).context("mgc.lock is not valid UTF-8")?;
    let lock: mgc_lockfile::Lockfile = mgc_lockfile::serialization::from_toml(&lock_content)?;
    let before = lock.packages.len();

    let (new_lock, merged) = merged_lockfile(&lock)?;
    let after = new_lock.packages.len();

    // Dry-run performs ZERO mutations: no cleanup, no write, no verify
    // spawn — pure report. (P0-2: dry-run tuyệt đối không sửa gì.)
    // (Dry-run: báo cáo thuần — không cleanup/write/verify.)
    if args.dry_run {
        let report = dry_run_report(before, after, merged);
        print_report(&args, &report)?;
        return Ok(());
    }

    let mut gc = GcReport::default();
    if merged > 0 {
        // Atomic commit (P0-2): the merged lock swaps in via tmp+rename
        // (never a torn write), verified by a runtime build, and rolled
        // back ATOMICALLY on verification failure. GC of the now
        // unreferenced store runs ONLY after the commit verifies — a
        // failed merge never deletes materialization it might need back.
        // (Commit nguyên tử → verify → mới GC; fail thì rollback nguyên tử.)
        // Re-seal the mutated lock: the digest must describe THIS body.
        // (Đóng dấu lại lock đã sửa: digest phải mô tả đúng body này.)
        let new_lock = mgc_lockfile::writer::with_lockfile_hash(&new_lock)?;
        let new_toml = mgc_lockfile::writer::serialize_lockfile(&new_lock)?;
        mgc_lockfile::ensure_lockfile_mutation_allowed(&mgc_lock)?;
        mgc_lockfile::atomic::atomic_write_locked(
            &guard,
            &mgc_lock,
            new_toml.as_bytes(),
            std::time::Duration::from_secs(60),
        )
        .map_err(|e| anyhow::anyhow!("dedupe lock commit failed: {e}"))?;
        if let Err(err) = verify_with_build(&project_root, false).await {
            mgc_lockfile::atomic::atomic_write_locked(
                &guard,
                &mgc_lock,
                lock_content.as_bytes(),
                std::time::Duration::from_secs(60),
            )
            .map_err(|e| anyhow::anyhow!("dedupe lock rollback failed: {e}"))?;
            bail!("merge rolled back — build verification failed: {err}");
        }
        gc = cleanup_unreferenced_vstore(&project_root, &new_lock);
        if gc.failed_count() > 0 {
            mgc_ui::warning(&format!(
                "dedupe GC partial: {} path(s) failed (lock commit already verified — no rollback needed): {}",
                gc.failed_count(),
                gc.failure_messages().join("; "),
            ));
        }
    }

    let report = DedupeReport {
        before_instances: before,
        after_instances: after,
        merged,
        disk_saved_bytes: gc.logical_payload_bytes_in_removed_paths(),
        entries: Vec::new(),
        gc_failed_paths: gc.failure_messages(),
    };
    print_report(&args, &report)?;
    Ok(())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
