//! Commit the update manifest through the existing mutation journal.
//! Ghi manifest update qua mutation journal hiện tại.

use super::super::*;

/// Preserve journal input and stage → write → post-image order.
/// Giữ input journal và thứ tự stage → write → post-image.
pub(crate) async fn commit_update_manifest(
    adapter: &dyn PackageAdapter,
    root: &Path,
    manifest: &Manifest,
    packages: &[String],
    snapshot: &MutationSnapshot,
    write_lock: &ProjectWriteLock,
) -> Result<()> {
    stage_mutation_journal(root, adapter, packages, snapshot, write_lock)?;
    journaled_step(
        adapter,
        root,
        snapshot,
        write_lock,
        "before-manifest-write",
        adapter.write_manifest(root, manifest),
        "after-manifest-write",
    )
    .await?;
    journaled_step(
        adapter,
        root,
        snapshot,
        write_lock,
        "before-post-image",
        async { record_post_image(root, manifest, write_lock) },
        "after-post-image",
    )
    .await?;
    Ok(())
}
