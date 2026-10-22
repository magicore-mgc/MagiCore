//! Test legacy journal adoption and untrusted payload handling.
//! Kiểm tra tiếp nhận journal cũ và xử lý payload không tin cậy.

use super::*;

#[tokio::test]
#[cfg(feature = "lib")]
async fn traversal_packages_field_is_inert() {
    // Journal packages chứa traversal không thể thoát ra ngoài: field
    // này không bao giờ thành path (chỉ diagnostic). Recovery vẫn chạy
    // bình thường với pre-image.
    // (Traversal-looking packages cannot escape — the field is inert.)
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    python_fixture(root);
    let (adapter, write_lock) = test_adapter_and_lock(root);
    let manifest = adapter.parse_manifest(root).await.unwrap();
    let snapshot =
        MutationSnapshot::capture(&manifest, root, &write_lock, MutationOperation::Remove).unwrap();
    stage_mutation_journal(
        root,
        &adapter,
        &["../../evil".to_string()],
        &snapshot,
        &write_lock,
    )
    .unwrap();
    let drifted = adapter.parse_manifest(root).await.unwrap();
    record_post_image(root, &drifted, &write_lock).unwrap();
    recover_interrupted_remove(&adapter, root, &write_lock)
        .await
        .unwrap();
    assert!(
        !root.parent().unwrap().join("evil").exists(),
        "no traversal side effect may exist"
    );
    assert!(
        !root
            .join(".magicore/journal/dependency-mutation/journal.json")
            .exists(),
        "journal must clear after normal recovery"
    );
}

#[tokio::test]
#[cfg(feature = "lib")]
async fn legacy_journal_path_is_adopted_then_recovered() {
    // P1-3: journal ở path cũ (journal/remove) được rename nguyên tử
    // sang path mới rồi phục hồi bình thường — không mất, không double.
    // (Legacy journal is adopted atomically, then recovered.)
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    python_fixture(root);
    let (adapter, write_lock) = test_adapter_and_lock(root);
    let manifest = adapter.parse_manifest(root).await.unwrap();
    let snapshot =
        MutationSnapshot::capture(&manifest, root, &write_lock, MutationOperation::Remove).unwrap();
    stage_mutation_journal(
        root,
        &adapter,
        &["attrs".to_string()],
        &snapshot,
        &write_lock,
    )
    .unwrap();
    // Rewind to the legacy layout (as an older binary left it).
    // (Giả journal của binary cũ ở path legacy.)
    let legacy = root.join(".magicore/journal/remove");
    let current = root.join(".magicore/journal/dependency-mutation");
    std::fs::rename(&current, &legacy).unwrap();

    adopt_legacy_journal(root, &write_lock).unwrap();
    recover_interrupted_remove(&adapter, root, &write_lock)
        .await
        .unwrap();
    assert!(!legacy.exists(), "legacy dir must be adopted away");
    assert!(
        !current.join("journal.json").exists(),
        "adopted journal must be recovered and cleared (current == pre)"
    );
    let after = adapter.parse_manifest(root).await.unwrap();
    assert_eq!(
        manifest_canonical_digest(&after),
        manifest_canonical_digest(&manifest)
    );
}

#[cfg(unix)]
#[tokio::test]
#[cfg(feature = "lib")]
async fn symlinked_legacy_marker_fails_closed() {
    // P1-3: marker legacy là symlink ra ngoài — adopt dời cả thư mục
    // nhưng recovery từ chối theo link, không đọc file ngoài.
    // (Symlinked legacy marker is refused at recovery, never followed.)
    use std::os::unix::fs::symlink;
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    python_fixture(root);
    let outside = tempfile::tempdir().unwrap();
    let evil = outside.path().join("evil.json");
    std::fs::write(&evil, r#"{"v":2}"#).unwrap();
    let legacy = root.join(".magicore/journal/remove");
    std::fs::create_dir_all(&legacy).unwrap();
    symlink(&evil, legacy.join("journal.json")).unwrap();

    // No pre-held guard here: begin() acquires its own (holding one
    // across the call would LockBusy against itself, not test recovery).
    // (Không giữ lock trước — begin tự acquire.)
    let adapter = mgc_lib_adapter::adapter_for(root, None, None)
        .unwrap()
        .expect("adapter");
    begin_dependency_mutation(&adapter, root, MutationOperation::Remove)
        .await
        .expect_err("symlinked legacy marker must fail closed");
    assert_eq!(
        std::fs::read(&evil).unwrap(),
        b"{\"v\":2}",
        "outside file must be untouched"
    );
}

#[tokio::test]
#[cfg(feature = "lib")]
async fn adopt_refuses_when_current_dir_has_garbage() {
    // P1-3: current dir tồn tại nhưng không marker + legacy có journal —
    // rename thất bại → lỗi fail-closed, không merge, không mất.
    // (Adopt refuses when the current dir is occupied without a marker.)
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    python_fixture(root);
    let (adapter, write_lock) = test_adapter_and_lock(root);
    let manifest = adapter.parse_manifest(root).await.unwrap();
    let snapshot =
        MutationSnapshot::capture(&manifest, root, &write_lock, MutationOperation::Remove).unwrap();
    stage_mutation_journal(
        root,
        &adapter,
        &["attrs".to_string()],
        &snapshot,
        &write_lock,
    )
    .unwrap();
    let legacy = root.join(".magicore/journal/remove");
    let current = root.join(".magicore/journal/dependency-mutation");
    std::fs::rename(&current, &legacy).unwrap();
    // Occupy the current dir with marker-less garbage.
    // (Chiếm dir mới bằng rác không marker.)
    std::fs::create_dir_all(&current).unwrap();
    std::fs::write(current.join("junk.txt"), "junk").unwrap();
    // Release the staging guard before re-acquiring below (same-process
    // flock is not re-entrant).
    // (Nhả guard staging trước khi acquire lại.)
    drop(write_lock);

    // begin acquires then adopt fails → whole entry fails closed.
    // (Adopt lỗi → entry lỗi, journal cũ còn nguyên.)
    let lock_result = mgc_lockfile::project_lock::ProjectWriteLock::acquire(
        root,
        std::time::Duration::from_secs(5),
    );
    assert!(lock_result.is_ok(), "lock itself must acquire");
    let guard = lock_result.unwrap();
    adopt_legacy_journal(root, &guard).expect_err("occupied current dir must refuse adoption");
    assert!(
        legacy.join("journal.json").exists(),
        "legacy journal must survive the refused adoption"
    );
    drop(guard);
}
