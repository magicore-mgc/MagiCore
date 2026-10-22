//! Rollback implementation.
//! Implementation rollback của shared mutation.

use crate::commands::core::shared::implementation::implementation::mutation_journal::implementation::mutation_journal_dir;
use std::path::Path;

use anyhow::Result;
use mgc_lockfile::project_lock::ProjectWriteLock;
use mgc_types::adapter::PackageAdapter;

use super::super::{
    MutationSnapshot, clear_mutation_journal, mark_journal_completed, restore_mutation_snapshot,
};

/// Roll a failed mutation tail back: restore manifest + lock from the
/// pre-edit snapshot (verified), disarm + clear the journal, and return
/// the tail error. If the restore itself fails the journal is KEPT for
/// the next run's recovery, and BOTH errors propagate in one combined
/// error (warning-only would hide a half-updated project).
/// Honest scope: manifest+lock files are restored; store/cache additions
/// made before the failure are additive-only and stay.
/// (Rollback manifest+lock CÓ VERIFY cho mọi op; restore lỗi thì GIỮ
/// journal + gộp cả 2 lỗi trả về.)
pub(crate) async fn rollback_mutation(
    adapter: &dyn PackageAdapter,
    root: &Path,
    snapshot: &MutationSnapshot,
    tail_error: impl std::fmt::Display,
    lock: &ProjectWriteLock,
) -> Result<()> {
    let op = snapshot.op;
    match restore_mutation_snapshot(adapter, root, snapshot, lock).await {
        Ok(()) => {
            // Disarm BEFORE clearing: even if the delete below fails or
            // the process dies here, the journal is completed and can
            // never roll back again.
            // (Vô hiệu TRƯỚC khi xóa — journal completed không rollback.)
            if let Err(e) = mark_journal_completed(root, lock) {
                return Err(combine_rollback_errors(
                    tail_error,
                    format!("{op} rollback restored files but disarming the journal failed: {e:#}"),
                ));
            }
            clear_mutation_journal(root, lock);
            mgc_ui::info(&format!(
                "{op} rolled back: manifest and lock restored (tail failed; store/cache additions, if any, are additive-only)",
            ));
            Err(anyhow::anyhow!("{tail_error:#}"))
        }
        Err(restore_error) => {
            // Journal is KEPT (not cleared) so the next run's recovery can
            // retry the restore — point the operator at it.
            // (GIỮ journal để lần chạy sau phục hồi — chỉ rõ đường dẫn.)
            let journal = mutation_journal_dir(root);
            Err(combine_rollback_errors(
                tail_error,
                format!(
                    "{restore_error:#} (crash journal kept at '{}' — fix the cause, then re-run to recover)",
                    journal.display()
                ),
            ))
        }
    }
}

/// Combine the install failure with a failed manifest restore into ONE
/// error carrying both (P0: a warning-only restore failure is invisible
/// to CI/API callers). Pure — unit-tested.
/// (Gộp 2 lỗi thành một — hàm thuần, có unit test.)
pub(crate) fn combine_rollback_errors(
    install_error: impl std::fmt::Display,
    restore_error: impl std::fmt::Display,
) -> anyhow::Error {
    anyhow::anyhow!(
        "mutation failed: {install_error:#} — AND the manifest rollback also failed: {restore_error:#} \
         (project may be half-updated: manifest dropped the dependency while lock/store still carry it — \
         re-add the package and retry)"
    )
}
