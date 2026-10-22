//! Completion implementation.
//! Implementation completion của shared mutation.

use std::path::Path;

use anyhow::Result;
use mgc_lockfile::project_lock::ProjectWriteLock;

use super::super::{
    JournalState, atomic_write_file, mutation_failpoint, mutation_journal_dir,
    parse_mutation_journal, read_project_string,
};

/// Flip a journal to completed (durable rewrite) BEFORE deleting it: a
/// crash between flip and delete leaves a completed journal, which
/// recovery only DELETES — stale cleanup never regains rollback rights
/// (P0: a leftover journal must not resurrect a successful remove).
/// (Đánh dấu completed TRƯỚC khi xóa — dọn sót không được rollback.)
pub(crate) fn mark_journal_completed(root: &Path, _lock: &ProjectWriteLock) -> Result<()> {
    mutation_failpoint("before-complete")?;
    let dir = mutation_journal_dir(root);
    let journal_path = dir.join("journal.json");
    // A missing journal here is ALWAYS a bug (stage runs before any
    // finish/mark caller) — say so explicitly instead of a bare ENOENT.
    // (Journal mất lúc disarm là bug — báo rõ.)
    if std::fs::symlink_metadata(&journal_path).is_err() {
        return Err(anyhow::anyhow!(
            "mutation journal missing at disarm time '{}' (stage was skipped — internal bug, report it)",
            journal_path.display()
        ));
    }
    let mut journal = parse_mutation_journal(&read_project_string(&journal_path)?, &journal_path)?;
    journal.state = JournalState::Completed;
    atomic_write_file(
        &dir,
        "journal.json",
        serde_json::to_string_pretty(&journal)?.as_bytes(),
    )?;
    mutation_failpoint("after-complete")?;
    Ok(())
}

/// Disarm (mark completed) then clear the journal on the success path.
/// A mark failure fails the op LOUDLY: leaving an in_progress journal
/// behind a success would let the next run roll back a finished op.
/// (Vô hiệu rồi xóa khi thành công — đánh dấu lỗi thì báo lỗi.)
pub(crate) fn finish_mutation_journal(root: &Path, lock: &ProjectWriteLock) -> Result<()> {
    mark_journal_completed(root, lock)?;
    clear_mutation_journal(root, lock);
    Ok(())
}

/// Best-effort journal-dir deletion, only ever called AFTER
/// mark_journal_completed: a leftover dir is state=completed and can
/// never trigger a restore — delete failure degrades to a warning plus
/// a same-state delete retry next run, never a rollback.
/// (Chỉ xóa sau khi completed — sót cũng không rollback.)
pub(crate) fn clear_mutation_journal(root: &Path, _lock: &ProjectWriteLock) {
    // Fault-injection point: simulate a cleanup failure (completed
    // journal lingers; the next run deletes it — never a rollback).
    // (Điểm lỗi cleanup — journal completed sót, lần sau xóa.)
    if mutation_failpoint("before-cleanup").is_err() {
        mgc_ui::warning(
            "injected fault at mutation phase 'before-cleanup' (MGC_MUTATION_FAILPOINT)",
        );
        return;
    }
    // let-chain edition 2024 — gộp điều kiện theo clippy 1.98.
    if let Err(e) = std::fs::remove_dir_all(mutation_journal_dir(root))
        && e.kind() != std::io::ErrorKind::NotFound
    {
        mgc_ui::warning(&format!("mutation journal cleanup failed: {e:#}"));
    }
}
