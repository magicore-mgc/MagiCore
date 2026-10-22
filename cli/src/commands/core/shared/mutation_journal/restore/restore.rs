//! Restore implementation.
//! Implementation restore của shared mutation.

use std::path::Path;

use anyhow::Result;
use mgc_lockfile::project_lock::ProjectWriteLock;
use mgc_types::adapter::PackageAdapter;

use super::super::{MutationSnapshot, manifest_canonical_digest, refuse_project_link};

/// Restore manifest + lock from a snapshot, VERIFIED: manifest by
/// canonical-digest equality after re-parse (project name+version, every
/// dependency with group+range+flags — never name-only), lock
/// byte-identical via the locked atomic writer (or absence restored).
/// Any mismatch is an error — a silent almost-restore is worse than a
/// loud failure.
/// (Phục hồi CÓ VERIFY: manifest digest chuẩn, lock nguyên tử đúng byte.)
pub(crate) async fn restore_mutation_snapshot(
    adapter: &dyn PackageAdapter,
    root: &Path,
    snapshot: &MutationSnapshot,
    lock: &ProjectWriteLock,
) -> Result<()> {
    adapter.write_manifest(root, &snapshot.manifest).await?;
    let reparsed = adapter.parse_manifest(root).await?;
    if manifest_canonical_digest(&reparsed) != manifest_canonical_digest(&snapshot.manifest) {
        return Err(anyhow::anyhow!(
            "manifest restore verify failed: re-parsed digest {:?} != snapshot {:?}",
            manifest_canonical_digest(&reparsed),
            manifest_canonical_digest(&snapshot.manifest)
        ));
    }
    let lock_path = root.join("mgc.lock");
    match &snapshot.lock_bytes {
        Some(bytes) => {
            mgc_lockfile::atomic::atomic_write_locked(
                lock,
                &lock_path,
                bytes,
                std::time::Duration::from_secs(60),
            )
            .map_err(|e| anyhow::anyhow!("lock restore failed: {e}"))?;
            let back = mgc_lockfile::read_lockfile_bytes(&lock_path)?;
            if back != *bytes {
                return Err(anyhow::anyhow!(
                    "lock restore verify failed: re-read bytes differ from snapshot"
                ));
            }
        }
        None => {
            refuse_project_link(&lock_path)?;
            if lock_path.exists() {
                std::fs::remove_file(&lock_path)?;
            }
            if lock_path.exists() {
                return Err(anyhow::anyhow!(
                    "lock restore verify failed: lock should be absent but still exists"
                ));
            }
        }
    }
    Ok(())
}
