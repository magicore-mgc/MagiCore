//! Test journal completion, corruption, absence and filesystem links.
//! Kiểm tra completion, corruption, trạng thái thiếu và symlink của journal.

use super::*;

#[tokio::test]
#[cfg(feature = "lib")]
async fn completed_journal_deletes_without_restoring() {
    // P0-A: a stale COMPLETED journal (success + failed cleanup) must
    // NEVER roll back — recovery only deletes it, files stay untouched.
    // (Journal completed sót: chỉ xóa, không restore.)
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
    // Op "succeeded" elsewhere, then cleanup failed: flip to completed
    // with a FOREIGN pid (as if staged by the dead process)…
    // (Giả op đã xong ở process khác: completed + pid lạ.)
    let journal_path = root.join(".magicore/journal/dependency-mutation/journal.json");
    let mut journal: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&journal_path).unwrap()).unwrap();
    journal["pid"] = serde_json::json!(u64::from(std::process::id()) + 1_000_000);
    journal["state"] = serde_json::json!("completed");
    std::fs::write(
        &journal_path,
        serde_json::to_string_pretty(&journal).unwrap(),
    )
    .unwrap();
    // …then the world drifts (new successful state). Recovery must NOT
    // resurrect the removed dep.
    std::fs::write(
        root.join("pyproject.toml"),
        "[project]\nname = \"m\"\nversion = \"0.1.0\"\ndependencies = [\"six==1.17.0\"]\n",
    )
    .unwrap();
    std::fs::write(root.join("mgc.lock"), "LOCK-NEW-STATE").unwrap();

    recover_interrupted_remove(&adapter, root, &write_lock)
        .await
        .unwrap();

    assert!(!journal_path.exists(), "completed journal must be deleted");
    let after = std::fs::read_to_string(root.join("pyproject.toml")).unwrap();
    assert!(
        !after.contains("attrs"),
        "removed dep must NOT be resurrected: {after}"
    );
    assert_eq!(
        std::fs::read(root.join("mgc.lock")).unwrap(),
        b"LOCK-NEW-STATE",
        "lock must stay untouched"
    );
}

#[tokio::test]
#[cfg(feature = "lib")]
async fn corrupt_journal_fails_closed() {
    // Journal hỏng: không đoán — lỗi fail-closed.
    // (Corrupt journal fails closed, never guesses.)
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    python_fixture(root);
    let (adapter, write_lock) = test_adapter_and_lock(root);
    let journal_dir = root.join(".magicore/journal/dependency-mutation");
    std::fs::create_dir_all(&journal_dir).unwrap();
    std::fs::write(journal_dir.join("journal.json"), "{not json").unwrap();
    recover_interrupted_remove(&adapter, root, &write_lock)
        .await
        .expect_err("corrupt journal must fail closed");
}

#[tokio::test]
#[cfg(feature = "lib")]
async fn missing_journal_is_a_noop() {
    // Không có journal: không làm gì.
    // (Missing journal is a no-op.)
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    python_fixture(root);
    let (adapter, write_lock) = test_adapter_and_lock(root);
    recover_interrupted_remove(&adapter, root, &write_lock)
        .await
        .unwrap();
    assert_eq!(
        std::fs::read(root.join("mgc.lock")).unwrap(),
        b"LOCK-BEFORE"
    );
}

#[cfg(unix)]
#[tokio::test]
#[cfg(feature = "lib")]
async fn symlinked_journal_dir_is_refused() {
    // Journal dir là symlink: từ chối, không ghi ra ngoài project.
    // (Symlinked journal dir is refused, never followed.)
    use std::os::unix::fs::symlink;
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    python_fixture(root);
    let (adapter, write_lock) = test_adapter_and_lock(root);
    let manifest = adapter.parse_manifest(root).await.unwrap();
    let snapshot =
        MutationSnapshot::capture(&manifest, root, &write_lock, MutationOperation::Remove).unwrap();
    let outside = tempfile::tempdir().unwrap();
    let journal_parent = root.join(".magicore/journal");
    std::fs::create_dir_all(&journal_parent).unwrap();
    symlink(outside.path(), journal_parent.join("dependency-mutation")).unwrap();
    stage_mutation_journal(
        root,
        &adapter,
        &["attrs".to_string()],
        &snapshot,
        &write_lock,
    )
    .expect_err("symlinked journal dir must be refused");
    assert!(
        !outside.path().join("journal.json").exists(),
        "nothing must be written outside the project"
    );
}

#[cfg(unix)]
#[tokio::test]
#[cfg(feature = "lib")]
async fn symlinked_journal_file_is_never_followed() {
    // journal.json là symlink trỏ ra ngoài: recovery từ chối, artifact
    // giữ nguyên (Item 2 — đọc sau khi chống, không trước).
    // (Symlinked journal.json is refused, never followed.)
    use std::os::unix::fs::symlink;
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
    // Swap journal.json for a link to an "in_progress" journal outside
    // the project — a naive read-then-validate would restore from it.
    // (Tráo journal.json bằng link ra ngoài — đọc ngây thơ sẽ dính.)
    let journal_path = root.join(".magicore/journal/dependency-mutation/journal.json");
    let outside = tempfile::tempdir().unwrap();
    let evil = outside.path().join("evil.json");
    std::fs::write(
        &evil,
        r#"{"v":1,"pid":999999,"packages":[],"lock_existed":false,"state":"in_progress"}"#,
    )
    .unwrap();
    std::fs::remove_file(&journal_path).unwrap();
    symlink(&evil, &journal_path).unwrap();
    recover_interrupted_remove(&adapter, root, &write_lock)
        .await
        .expect_err("symlinked journal.json must be refused");
    // Nothing followed the link: project files intact, outside untouched
    // beyond our own setup write.
    // (Không theo link: file project nguyên, ngoài không bị động.)
    assert_eq!(
        std::fs::read(root.join("mgc.lock")).unwrap(),
        b"LOCK-BEFORE"
    );
    let after = adapter.parse_manifest(root).await.unwrap();
    assert_eq!(
        manifest_canonical_digest(&after),
        manifest_canonical_digest(&manifest)
    );
}

#[cfg(unix)]
#[tokio::test]
async fn symlinked_magicore_parent_is_refused() {
    // .magicore là symlink: acquire lock và staging journal đều từ chối
    // (Item 4 — parent gap đã đóng).
    // (Symlinked .magicore is refused by lock acquire and journal write.)
    use std::os::unix::fs::symlink;
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    python_fixture(root);
    let outside = tempfile::tempdir().unwrap();
    symlink(outside.path(), root.join(".magicore")).unwrap();
    mgc_lockfile::project_lock::ProjectWriteLock::acquire(root, std::time::Duration::from_secs(5))
        .expect_err("symlinked .magicore must refuse the writer lock");
}
