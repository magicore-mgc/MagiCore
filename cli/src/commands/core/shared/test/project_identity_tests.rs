//! Test project lineage, identity and core ownership boundaries.
//! Kiểm tra lineage, identity project và ranh giới core sở hữu.

use super::*;

#[tokio::test]
#[cfg(feature = "lib")]
async fn foreign_core_journal_is_never_restored() {
    // P0-3: journal của core khác (web) đem sang project lib — recovery
    // phải từ chối, không được lấy adapter hiện tại làm authority.
    // (Foreign-core journal fails closed — never restored cross-core.)
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    python_fixture(root);
    let (adapter, write_lock) = test_adapter_and_lock(root);
    let journal_dir = root.join(".magicore/journal/dependency-mutation");
    std::fs::create_dir_all(&journal_dir).unwrap();
    let canonical = root.canonicalize().unwrap().display().to_string();
    let journal = serde_json::json!({
        "v": 4,
        "op": "remove",
        "pid": 12345,
        "packages": ["some-web-dep"],
        "lock_existed": true,
        "identity": {
            "core": "app",
            "language": "flutter",
            "format": "pubspec.yaml",
            "relpath": "pubspec.yaml",
        },
        "project_root": canonical,
        "project_id": "00000000-0000-4000-8000-000000000000",
        "pre_digest": [],
        "post_digest": null,
        "state": "in_progress",
    });
    std::fs::write(
        journal_dir.join("journal.json"),
        serde_json::to_string_pretty(&journal).unwrap(),
    )
    .unwrap();
    // This project has a DIFFERENT real identity (its own project.id).
    // (Project này có UUID thật khác — journal ngoại lai.)
    std::fs::write(
        root.join(".magicore/project.id"),
        "11111111-1111-4111-8111-111111111111",
    )
    .unwrap();
    std::fs::write(
        journal_dir.join("journal.json"),
        serde_json::to_string_pretty(&journal).unwrap(),
    )
    .unwrap();
    let err = recover_interrupted_remove(&adapter, root, &write_lock)
        .await
        .expect_err("foreign-core journal must fail closed");
    let msg = format!("{err:#}");
    assert!(
        msg.contains("app") && msg.contains("foreign"),
        "must name the mismatch, not silently restore: {msg}"
    );
    let manifest = std::fs::read_to_string(root.join("pyproject.toml")).unwrap();
    assert!(
        manifest.contains("attrs"),
        "current manifest must be untouched:\n{manifest}"
    );
}

#[tokio::test]
#[cfg(feature = "lib")]
async fn copied_project_journal_recovers_same_lineage() {
    // P1-2: copy cả project (UUID đi cùng) — lineage giống nhau, state
    // giống nhau → recovery thành công đúng đắn (không phải ngoại lai).
    // Khác với project KHÁC UUID (test foreign) luôn bị từ chối.
    // (Copied project shares lineage — recovery succeeds correctly.)
    let dir_a = tempfile::tempdir().unwrap();
    python_fixture(dir_a.path());
    let (adapter_a, lock_a) = test_adapter_and_lock(dir_a.path());
    let manifest = adapter_a.parse_manifest(dir_a.path()).await.unwrap();
    let snapshot =
        MutationSnapshot::capture(&manifest, dir_a.path(), &lock_a, MutationOperation::Remove)
            .unwrap();
    stage_mutation_journal(
        dir_a.path(),
        &adapter_a,
        &["attrs".to_string()],
        &snapshot,
        &lock_a,
    )
    .unwrap();
    drop(lock_a);
    // Copy the whole tree (journal + project.id included).
    // (Copy toàn bộ cây — journal + UUID đi cùng.)
    let dir_b = tempfile::tempdir().unwrap();
    copy_dir_recursive(dir_a.path(), dir_b.path()).unwrap();
    let (adapter_b, lock_b) = test_adapter_and_lock(dir_b.path());
    recover_interrupted_remove(&adapter_b, dir_b.path(), &lock_b)
        .await
        .unwrap();
    let after = adapter_b.parse_manifest(dir_b.path()).await.unwrap();
    assert_eq!(
        manifest_canonical_digest(&after),
        manifest_canonical_digest(&manifest),
        "copied lineage restores identically"
    );
    assert!(
        !dir_b
            .path()
            .join(".magicore/journal/dependency-mutation/journal.json")
            .exists(),
        "journal must clear after recovery"
    );
    drop(lock_b);
}

#[tokio::test]
#[cfg(feature = "lib")]
async fn moved_project_recovers_with_notice() {
    // P1-2: rename/move checkout (UUID đi cùng) — recovery thành công
    // (move hợp lệ, khác với copy-sang-project-lạ).
    // (Moved checkout recovers — same UUID lineage.)
    let outer = tempfile::tempdir().unwrap();
    let root_a = outer.path().join("a");
    std::fs::create_dir_all(&root_a).unwrap();
    python_fixture(&root_a);
    std::fs::write(root_a.join("mgc.lock"), "LOCK-BEFORE").unwrap();
    let (adapter, lock) = test_adapter_and_lock(&root_a);
    let manifest = adapter.parse_manifest(&root_a).await.unwrap();
    let snapshot =
        MutationSnapshot::capture(&manifest, &root_a, &lock, MutationOperation::Remove).unwrap();
    stage_mutation_journal(&root_a, &adapter, &["attrs".to_string()], &snapshot, &lock).unwrap();
    drop(lock);
    let root_b = outer.path().join("b");
    std::fs::rename(&root_a, &root_b).unwrap();

    let (adapter_b, lock_b) = test_adapter_and_lock(&root_b);
    recover_interrupted_remove(&adapter_b, &root_b, &lock_b)
        .await
        .unwrap();
    let after = adapter_b.parse_manifest(&root_b).await.unwrap();
    assert_eq!(
        manifest_canonical_digest(&after),
        manifest_canonical_digest(&manifest)
    );
    drop(lock_b);
}

#[tokio::test]
#[cfg(feature = "lib")]
async fn missing_project_id_fails_closed() {
    // P1-2: có journal nhưng mất project.id (thư mục bị thay/khuyết) —
    // fail-closed, không đoán lineage.
    // (Missing project.id with a staged journal fails closed.)
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
    std::fs::remove_file(root.join(".magicore/project.id")).unwrap();
    recover_interrupted_remove(&adapter, root, &write_lock)
        .await
        .expect_err("missing project.id must fail closed");
    drop(write_lock);
}

#[tokio::test]
#[cfg(feature = "lib")]
async fn relpath_participates_in_identity() {
    // Blocker 1: cùng core/language/format nhưng khác relpath vẫn
    // mismatch — relpath có tham gia so sánh (không phải trường trang trí).
    // (relpath mismatch fails even when everything else matches.)
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    python_fixture(root);
    let (adapter, write_lock) = test_adapter_and_lock(root);
    craft_identity_journal(root, "lib", "python", "pyproject.toml", "other.toml");
    recover_interrupted_remove(&adapter, root, &write_lock)
        .await
        .expect_err("relpath mismatch must fail closed");
    drop(write_lock);
}
