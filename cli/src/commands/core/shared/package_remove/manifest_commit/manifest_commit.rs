//! Stage the remove journal, write the manifest and record its post-image.
//! Stage journal remove, ghi manifest và lưu post-image.

use super::super::*;

/// Preserve timer scope and journal → write → post-image order.
/// Giữ phạm vi timer và thứ tự journal → write → post-image.
pub(crate) async fn commit_removed_manifest(
    adapter: &dyn PackageAdapter,
    root: &Path,
    packages: &[String],
    manifest: &Manifest,
    snapshot: &MutationSnapshot,
    write_lock: &ProjectWriteLock,
) -> Result<()> {
    let write_started_at = std::time::Instant::now();
    // Commit point: journal first; the manifest has not changed before this point.
    // (Điểm commit: ghi journal trước; manifest chưa đổi trước bước này.)
    stage_mutation_journal(root, adapter, packages, snapshot, write_lock)?;
    // Route staged-write failures through the existing journal boundary.
    // (Đưa lỗi ghi sau khi stage qua journal boundary hiện tại.)
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
    profile_install_mark("remove_write_manifest", write_started_at);
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
