//! Test crash recovery, drift detection and project isolation.
//! Kiểm tra phục hồi crash, phát hiện drift và cô lập project.

use super::*;

#[tokio::test]
#[cfg(feature = "lib")]
async fn interrupted_remove_journal_recovers_manifest_and_lock() {
    // Hermetic crash-recovery test (NO registry): stage a journal, run
    // the op's own write + post-image, then "crash" — a fresh process
    // (flag dead, pid ignored) must restore the pre-image manifest
    // digest and the byte-identical lock, then clear the journal.
    // (Test phục hồi crash không cần mạng: pid trên đĩa bị lờ đi.)
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    std::fs::write(
        root.join("pyproject.toml"),
        "[project]\nname = \"m\"\nversion = \"0.1.0\"\nrequires-python = \">=3.11\"\ndependencies = [\"six==1.17.0\", \"attrs==23.1.0\"]\n",
    )
    .unwrap();
    std::fs::write(root.join("mgc.lock"), "LOCK-BEFORE").unwrap();
    let adapter = mgc_lib_adapter::adapter_for(root, None, None)
        .unwrap()
        .expect("pyproject must detect a python lib adapter");
    let write_lock = ProjectWriteLock::acquire(root, std::time::Duration::from_secs(30)).unwrap();
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
    // Simulate the op's own write + crash AFTER it: drift the files,
    // record the post-image (exactly what the op does after writing),
    // then die. A fresh process (flag cleared, pid ignored) must
    // recover to the pre-image.
    // (Giả crash sau khi op tự ghi: drift + post-image, rồi chết.)
    std::fs::write(
        root.join("pyproject.toml"),
        "[project]\nname = \"m\"\nversion = \"0.1.0\"\ndependencies = []\n",
    )
    .unwrap();
    std::fs::write(root.join("mgc.lock"), "LOCK-DRIFTED").unwrap();
    let drifted = adapter.parse_manifest(root).await.unwrap();
    record_post_image(root, &drifted, &write_lock).unwrap();
    let journal_path = root.join(".magicore/journal/dependency-mutation/journal.json");

    recover_interrupted_remove(&adapter, root, &write_lock)
        .await
        .unwrap();

    let after = adapter.parse_manifest(root).await.unwrap();
    assert_eq!(
        manifest_canonical_digest(&after),
        manifest_canonical_digest(&manifest),
        "manifest must restore to the same canonical digest (project, groups, ranges, flags)"
    );
    assert_eq!(
        std::fs::read(root.join("mgc.lock")).unwrap(),
        b"LOCK-BEFORE",
        "lock must restore byte-identical"
    );
    assert!(
        !journal_path.exists(),
        "journal must be cleared after recovery"
    );
}

#[tokio::test]
#[cfg(feature = "lib")]
async fn user_edit_after_crash_fails_closed() {
    // User sửa manifest sau crash (không khớp pre/post): recovery LỖI,
    // không ghi đè việc của user; artifact giữ nguyên.
    // (Post-crash user edits fail closed — never overwritten.)
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
    let drifted = adapter.parse_manifest(root).await.unwrap();
    record_post_image(root, &drifted, &write_lock).unwrap();
    // A human edits the manifest after the crash (third state).
    // (User sửa tay sau crash — trạng thái thứ ba.)
    std::fs::write(
        root.join("pyproject.toml"),
        "[project]\nname = \"m\"\nversion = \"0.1.0\"\ndependencies = [\"six==1.17.0\", \"attrs==23.1.0\", \"human-edit==1.0.0\"]\n",
    )
    .unwrap();

    recover_interrupted_remove(&adapter, root, &write_lock)
        .await
        .expect_err("user edits after crash must fail closed");
    let kept = std::fs::read_to_string(root.join("pyproject.toml")).unwrap();
    assert!(
        kept.contains("human-edit"),
        "user edit must survive the refused recovery:\n{kept}"
    );
}

#[tokio::test]
#[cfg(feature = "lib")]
async fn missing_manifest_backup_fails_closed() {
    // Journal in_progress nhưng mất backup: lỗi, không đoán từ manifest
    // hiện tại (pre-image đã mất).
    // (Missing backup fails closed — the pre-image is gone.)
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
    std::fs::remove_file(root.join(".magicore/journal/dependency-mutation/manifest.json")).unwrap();

    recover_interrupted_remove(&adapter, root, &write_lock)
        .await
        .expect_err("missing backup must fail closed");
}

#[tokio::test]
#[cfg(feature = "lib")]
async fn staged_journal_in_project_a_never_blocks_project_b() {
    // P0-2: không còn cờ global — project A giữ journal đang mở, project
    // B vẫn stage/post/finish bình thường trong CÙNG process (MCP,
    // monorepo, test runner). Lock OS riêng từng project nên không chặn nhau.
    // (An open journal in A never affects B in the same process.)
    let dir_a = tempfile::tempdir().unwrap();
    let dir_b = tempfile::tempdir().unwrap();
    python_fixture(dir_a.path());
    python_fixture(dir_b.path());
    let (adapter_a, lock_a) = test_adapter_and_lock(dir_a.path());
    let (adapter_b, lock_b) = test_adapter_and_lock(dir_b.path());
    let manifest_a = adapter_a.parse_manifest(dir_a.path()).await.unwrap();
    let snapshot_a = MutationSnapshot::capture(
        &manifest_a,
        dir_a.path(),
        &lock_a,
        MutationOperation::Remove,
    )
    .unwrap();
    // A stages and HOLDS (guard + journal stay alive).
    // (A stage rồi giữ — không finish.)
    stage_mutation_journal(
        dir_a.path(),
        &adapter_a,
        &["attrs".to_string()],
        &snapshot_a,
        &lock_a,
    )
    .unwrap();
    // B runs a full stage → post → finish cycle unaffected.
    // (B chạy trọn vòng không ảnh hưởng.)
    let manifest_b = adapter_b.parse_manifest(dir_b.path()).await.unwrap();
    let snapshot_b = MutationSnapshot::capture(
        &manifest_b,
        dir_b.path(),
        &lock_b,
        MutationOperation::Remove,
    )
    .unwrap();
    stage_mutation_journal(
        dir_b.path(),
        &adapter_b,
        &["attrs".to_string()],
        &snapshot_b,
        &lock_b,
    )
    .unwrap();
    record_post_image(dir_b.path(), &manifest_b, &lock_b).unwrap();
    finish_mutation_journal(dir_b.path(), &lock_b).unwrap();
    assert!(
        !dir_b
            .path()
            .join(".magicore/journal/dependency-mutation/journal.json")
            .exists(),
        "B's journal must clear"
    );
    assert!(
        dir_a
            .path()
            .join(".magicore/journal/dependency-mutation/journal.json")
            .exists(),
        "A's held journal must be untouched by B"
    );
    drop(lock_a);
}

#[tokio::test]
#[cfg(feature = "lib")]
async fn aborted_task_leaves_recoverable_journal() {
    // Hủy task giữa mutation (panic/cancel): guard rớt theo future,
    // lock OS tự nhả, journal ở lại cho gateway entry sau (không còn
    // ownership in-memory nào để leak).
    // (Aborted task leaves a recoverable journal, no leaked ownership.)
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().to_path_buf();
    python_fixture(&root);
    // This recovery test needs to exercise an absent pre-image, not an
    // intentionally malformed lockfile used by byte-rollback tests.
    // (Bài test recovery này kiểm tra pre-image không có lockfile.)
    std::fs::remove_file(root.join("mgc.lock")).unwrap();
    let root_clone = root.clone();
    let handle = tokio::spawn(async move {
        let adapter = mgc_lib_adapter::adapter_for(&root_clone, None, None)
            .unwrap()
            .expect("adapter");
        // Full gateway entry (acquire + recover), then stage, then hang
        // forever mid-mutation — the abort below simulates SIGKILL.
        // (Entry gateway thật rồi treo giữa mutation — abort giả SIGKILL.)
        let guard = begin_dependency_mutation(&adapter, &root_clone, MutationOperation::Remove)
            .await
            .unwrap();
        let manifest = adapter.parse_manifest(&root_clone).await.unwrap();
        let snapshot =
            MutationSnapshot::capture(&manifest, &root_clone, &guard, MutationOperation::Remove)
                .unwrap();
        stage_mutation_journal(
            &root_clone,
            &adapter,
            &["attrs".to_string()],
            &snapshot,
            &guard,
        )
        .unwrap();
        futures_util::future::pending::<()>().await;
        #[allow(unreachable_code)]
        drop(guard);
    });
    // Let the task stage, then abort it mid-mutation.
    // (Cho task stage xong rồi hủy giữa chừng.)
    let mut staged = false;
    for _ in 0..100 {
        tokio::time::sleep(std::time::Duration::from_millis(20)).await;
        if root
            .join(".magicore/journal/dependency-mutation/journal.json")
            .exists()
        {
            staged = true;
            break;
        }
    }
    assert!(staged, "task must stage before abort");
    handle.abort();
    let _ = handle.await;

    // A fresh gateway entry acquires (lock auto-released on abort),
    // recovers the staged journal (current == pre: nothing was written),
    // and proceeds — journal gone afterwards.
    // (Entry mới acquire, phục hồi journal dở, rồi đi tiếp.)
    let adapter = mgc_lib_adapter::adapter_for(&root, None, None)
        .unwrap()
        .expect("adapter");
    begin_dependency_mutation(&adapter, &root, MutationOperation::Remove)
        .await
        .expect("fresh entry must acquire and recover after abort");
    assert!(
        !root
            .join(".magicore/journal/dependency-mutation/journal.json")
            .exists(),
        "aborted op's journal must be recoverable by the next entry"
    );
}
