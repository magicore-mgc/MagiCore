//! Step implementation.
//! Implementation step của shared mutation.

use std::path::Path;

use anyhow::Result;
use mgc_lockfile::project_lock::ProjectWriteLock;
use mgc_types::adapter::PackageAdapter;

use super::super::{MutationSnapshot, mutation_failpoint, rollback_mutation};

/// Run one post-stage mutation step with correctly ordered failpoints:
/// before-check, work, after-check — ANY error rolls the op back to its
/// pre-image instead of leaking a staged journal. Phase names mean what
/// they say: `before-manifest-write` fires BEFORE the write,
/// `after-manifest-write` fires AFTER it (proving rollback of a
/// completed write, not a skipped one).
/// (Failpoint đúng thứ tự before/work/after — tên đúng nghĩa.)
pub(crate) async fn journaled_step<E>(
    adapter: &dyn PackageAdapter,
    root: &Path,
    snapshot: &MutationSnapshot,
    lock: &ProjectWriteLock,
    before_phase: &str,
    work: impl std::future::Future<Output = Result<(), E>>,
    after_phase: &str,
) -> Result<()>
where
    E: std::fmt::Display,
{
    if let Err(e) = mutation_failpoint(before_phase) {
        return rollback_mutation(adapter, root, snapshot, e, lock).await;
    }
    if let Err(e) = work.await {
        return rollback_mutation(adapter, root, snapshot, e, lock).await;
    }
    if let Err(e) = mutation_failpoint(after_phase) {
        return rollback_mutation(adapter, root, snapshot, e, lock).await;
    }
    Ok(())
}
