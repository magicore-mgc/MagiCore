use super::*;

#[allow(dead_code)]
pub async fn remove(
    adapter: &dyn PackageAdapter,
    root: &Path,
    packages: Vec<String>,
    install: bool,
) -> Result<()> {
    super::validation::validate_remove_request(packages.len())?;
    ensure_native_manifest_owner(adapter.manifest_owned(), "remove")?;
    info(&format!("Removing {} package(s)...", packages.len()));
    // Single mutation gateway: lock + stale-journal recovery. Direct
    // lock acquisition that forgets recovery is a bug (P0).
    // (Qua cổng gateway duy nhất — không tự acquire rồi quên recovery.)
    let write_lock = begin_dependency_mutation(adapter, root, MutationOperation::Remove).await?;
    let parse_started_at = std::time::Instant::now();
    let mut manifest = adapter.parse_manifest(root).await?;
    profile_install_mark("remove_parse_manifest", parse_started_at);
    // Best-effort rollback scope (NOT full store-level atomicity — the
    // install tail re-resolves instead of using the pruned graph, and
    // store/cache additions are additive-only): snapshot the manifest
    // struct + mgc.lock bytes BEFORE the edit, and stage a crash journal
    // so a failure below (or a SIGKILL before the tail finishes) can
    // restore both files. Must capture BEFORE the remove loop mutates.
    // (Snapshot + journal TRƯỚC khi sửa — rollback/journal phục hồi cả
    // manifest lẫn lock; store/cache chỉ thêm, không xóa.)
    let snapshot =
        MutationSnapshot::capture(&manifest, root, &write_lock, MutationOperation::Remove)?;
    let removed_any =
        super::dependency_removal::remove_requested_dependencies(&mut manifest, &packages)?;
    if !removed_any {
        return super::post_remove_install::finish_no_removal(install);
    }
    super::manifest_commit::commit_removed_manifest(
        adapter,
        root,
        &packages,
        &manifest,
        &snapshot,
        &write_lock,
    )
    .await?;
    super::post_remove_install::finish_remove(adapter, root, install, &snapshot, &write_lock).await
}
