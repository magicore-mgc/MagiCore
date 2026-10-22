//! Commit the changed manifest through its existing journal boundary.
//! Commit manifest đã đổi qua journal boundary hiện tại.

use super::super::*;

/// Preserve stage → write → post-image order and return journal ownership.
/// Giữ thứ tự stage → write → post-image và trả trạng thái journal.
#[allow(clippy::too_many_arguments)]
pub(crate) async fn commit_manifest_changes(
    adapter: &dyn PackageAdapter,
    root: &Path,
    no_save: bool,
    changed_any: bool,
    manifest_after_add: &Option<Manifest>,
    add_snapshot: &Option<MutationSnapshot>,
    added_packages: &[AddedPackage],
    write_lock: &ProjectWriteLock,
) -> Result<bool> {
    let mut add_journaled = false;
    if !no_save {
        if changed_any {
            if let Some(manifest) = manifest_after_add.as_ref() {
                if let Some(snapshot) = add_snapshot.as_ref() {
                    stage_mutation_journal(
                        root,
                        adapter,
                        &added_packages
                            .iter()
                            .map(|added| added.id.name_str().to_string())
                            .collect::<Vec<_>>(),
                        snapshot,
                        write_lock,
                    )?;
                    add_journaled = true;
                }
                let write_started_at = std::time::Instant::now();
                // Journaled (mgc-owned manifest) writes route every error
                // through rollback; unjournaled (toolchain-owned) writes
                // keep the legacy direct error (the tool owns that file).
                // (Chỉ đường journal khi có journal — tool-owned giữ cũ.)
                if let (true, Some(snapshot)) = (add_journaled, add_snapshot.as_ref()) {
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
                } else {
                    adapter.write_manifest(root, manifest).await?;
                }
                profile_install_mark("add_write_manifest", write_started_at);
                if let (true, Some(snapshot)) = (add_journaled, add_snapshot.as_ref()) {
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
                }
            }
        } else {
            info("Manifest unchanged.");
        }
    }
    Ok(add_journaled)
}
