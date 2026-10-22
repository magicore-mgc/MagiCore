//! Crash recovery and lock snapshot/restore tests.
//! Test phục hồi crash và snapshot/restore lock.

use std::path::PathBuf;

fn tmp(tag: &str) -> PathBuf {
    let dir =
        std::env::temp_dir().join(format!("mgc-install-recovery-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// Nợ 2-RED: stale snapshot dirs (`.mgc-snap-*`, left by SIGKILLed
/// installs — TempDir never cleans on kill) must be swept at the next
/// install start. Safe: the project install lock forbids concurrent
/// installs in one project, so every prefixed dir here is garbage.
/// Thư mục snapshot cũ (SIGKILL) phải được quét dọn ở lần install sau.
#[test]
fn sweep_stale_snapshots_removes_only_prefixed_dirs() {
    use crate::install::sweep_stale_snapshots;
    let dir = tmp("sweep");
    std::fs::create_dir_all(dir.join(".mgc-snap-deadbeef")).unwrap();
    std::fs::write(dir.join(".mgc-snap-deadbeef").join("x"), b"stale").unwrap();
    std::fs::create_dir_all(dir.join("node_modules")).unwrap();
    std::fs::write(dir.join("mgc.toml"), "name = \"x\"\n").unwrap();
    let swept = sweep_stale_snapshots(&dir);
    assert_eq!(swept, 1, "one stale snapshot must go");
    assert!(!dir.join(".mgc-snap-deadbeef").exists());
    assert!(dir.join("node_modules").exists(), "real tree untouched");
    assert!(dir.join("mgc.toml").is_file(), "files untouched");
}

/// P1-RED (sweep): a hostile `.mgc-snap-*` SYMLINK must not cause
/// deletion outside the project — and counts nothing.
/// Symlink `.mgc-snap-*` độc hại không được gây xóa ngoài project.
#[cfg(unix)]
#[test]
fn sweep_never_follows_hostile_snapshot_symlink() {
    use crate::install::sweep_stale_snapshots;
    let dir = tmp("sweep-link");
    let outside = tmp("sweep-victim");
    std::fs::write(outside.join("precious.txt"), b"keep").unwrap();
    std::os::unix::fs::symlink(&outside, dir.join(".mgc-snap-evil")).unwrap();
    let swept = sweep_stale_snapshots(&dir);
    assert_eq!(swept, 0, "symlink must not count as swept");
    assert!(
        outside.join("precious.txt").is_file(),
        "outside target must survive"
    );
    assert!(
        std::fs::symlink_metadata(dir.join(".mgc-snap-evil"))
            .unwrap()
            .file_type()
            .is_symlink(),
        "the hostile link itself is left alone"
    );
}

/// P0-1-RED: pre-install lock bytes come back after failure; absent
/// prior means the failed write is removed (fail-closed, never silent).
/// Lock cũ về sau fail; chưa từng có thì xóa file đã ghi hỏng.
/// P0-2-RED: TreeBackup take/rollback/commit + Drop auto-restore; crash
/// recovery restores backups and sweeps staging litter.
/// Guard take/rollback/commit + Drop tự khôi phục; recovery dựng backup.
#[test]
fn restore_prior_lock_roundtrips() {
    use crate::install::{restore_prior_lock_result, snapshot_prior_lock};
    let dir = tmp("restore");
    let path = dir.join("mgc.lock");
    std::fs::write(&path, b"v1-bytes").unwrap();
    assert_eq!(
        snapshot_prior_lock(&dir).unwrap(),
        Some(b"v1-bytes".to_vec())
    );
    restore_prior_lock_result(&dir, &Some(b"v0-bytes".to_vec())).unwrap();
    assert_eq!(std::fs::read(&path).unwrap(), b"v0-bytes");
    restore_prior_lock_result(&dir, &None).unwrap();
    assert!(
        !path.exists(),
        "no prior lock means the failed write is removed"
    );
    restore_prior_lock_result(&dir, &None).unwrap();
}

#[test]
fn prior_lock_snapshot_distinguishes_missing_from_invalid_or_unreadable_paths() {
    use crate::install::snapshot_prior_lock;

    let missing = tmp("snapshot-missing");
    assert_eq!(snapshot_prior_lock(&missing).unwrap(), None);

    let directory_lock = tmp("snapshot-directory");
    std::fs::create_dir(directory_lock.join("mgc.lock")).unwrap();
    assert!(
        snapshot_prior_lock(&directory_lock).is_err(),
        "a directory named mgc.lock must not be treated as an absent lock"
    );
}

#[cfg(unix)]
#[test]
fn prior_lock_snapshot_refuses_symlink_without_reading_target() {
    use crate::install::snapshot_prior_lock;

    let project = tmp("snapshot-symlink");
    let outside = tmp("snapshot-outside").join("precious.lock");
    std::fs::write(&outside, b"external-lock-bytes").unwrap();
    std::os::unix::fs::symlink(&outside, project.join("mgc.lock")).unwrap();

    assert!(snapshot_prior_lock(&project).is_err());
    assert_eq!(std::fs::read(&outside).unwrap(), b"external-lock-bytes");
}

#[test]
fn recover_interrupted_install_restores_and_sweeps() {
    use crate::install::recover_interrupted_install;
    let dir = tmp("recover");
    let live = dir.join("node_modules");
    // Simulate SIGKILL mid-install: stale staging + backup, no live tree.
    let staging = dir.join(".mgc-stage-dead");
    std::fs::create_dir_all(&staging).unwrap();
    let backup = dir.join(".mgc-prev-1-2-3");
    std::fs::create_dir_all(backup.join("pkg")).unwrap();
    std::fs::write(backup.join("pkg").join("a.txt"), b"orig").unwrap();
    // Hostile symlink must be refused, never restored.
    #[cfg(unix)]
    std::os::unix::fs::symlink("/etc", dir.join(".mgc-prev-evil")).unwrap();

    let out = recover_interrupted_install(&dir, &live, &dir).unwrap();
    assert!(out.restored_backup, "backup must restore");
    assert_eq!(out.removed_staging, 1, "staging litter swept");
    assert_eq!(
        std::fs::read(live.join("pkg").join("a.txt")).unwrap(),
        b"orig"
    );
    assert!(!staging.exists());
    // Second run is a no-op (idempotent).
    let out2 = recover_interrupted_install(&dir, &live, &dir).unwrap();
    assert!(!out2.restored_backup);
    assert_eq!(out2.removed_staging, 0);
}

/// Disk-full / unwritable-destination plumbing (P0-store): write
/// failures surface as Err, never panic, never silent success — the
/// same io::Error path ENOSPC takes.
/// (Lỗi ghi trả Err, không panic, không thành công giả — cùng đường
/// io::Error mà ENOSPC đi.)
#[test]
fn lock_restore_write_failure_is_an_error_not_panic() {
    use crate::install::restore_prior_lock_result;
    let dir = tmp("nowrite");
    // A regular FILE as project root: join() yields file/mgc.lock and
    // every write fails deterministically on all platforms.
    // (File thường làm root: mọi write fail tất định mọi nền tảng.)
    let file_as_root = dir.join("not-a-dir");
    std::fs::write(&file_as_root, b"x").unwrap();
    assert!(
        restore_prior_lock_result(&file_as_root, &Some(b"v0".to_vec())).is_err(),
        "unwritable lock destination must Err"
    );
}

#[test]
fn multiple_backup_candidates_fail_closed_without_deleting_them() {
    use crate::install::recover_interrupted_install;

    let dir = tmp("recover-preserve-backups");
    let live = dir.join("node_modules");
    let candidate_one = dir.join(".mgc-prev-one");
    let candidate_two = dir.join(".mgc-prev-two");
    std::fs::create_dir_all(candidate_one.join("pkg")).unwrap();
    std::fs::write(
        candidate_one.join("pkg").join("state.txt"),
        b"first-recovery-candidate",
    )
    .unwrap();
    std::fs::create_dir_all(candidate_two.join("pkg")).unwrap();
    std::fs::write(
        candidate_two.join("pkg").join("state.txt"),
        b"second-recovery-candidate",
    )
    .unwrap();

    // Multiple backups have ambiguous history. Recovery must not choose by
    // timestamp or delete either candidate, regardless of the live path.
    std::fs::write(&live, b"unexpected-live-file").unwrap();
    let error = recover_interrupted_install(&dir, &live, &dir)
        .expect_err("unrecoverable live path must stop install recovery");

    assert!(
        error
            .to_string()
            .contains("refusing to choose by timestamp")
    );
    assert!(
        candidate_one.is_dir(),
        "first candidate must survive ambiguity"
    );
    assert!(
        candidate_two.is_dir(),
        "second candidate must survive ambiguity"
    );
    assert_eq!(std::fs::read(&live).unwrap(), b"unexpected-live-file");
}

#[test]
fn failed_single_backup_restore_is_reported_and_backup_is_preserved() {
    use crate::install::recover_interrupted_install;

    let dir = tmp("recover-preserve-single");
    let live = dir.join("node_modules");
    let backup = dir.join(".mgc-prev-single");
    std::fs::create_dir_all(backup.join("pkg")).unwrap();
    std::fs::write(backup.join("pkg").join("state.txt"), b"recoverable-state").unwrap();
    std::fs::write(&live, b"unexpected-live-file").unwrap();

    let error = recover_interrupted_install(&dir, &live, &dir)
        .expect_err("failed restore must prevent the install from continuing");

    assert!(
        error
            .to_string()
            .contains("cannot clear interrupted install path")
    );
    assert!(backup.is_dir(), "failed restore must preserve its backup");
    assert_eq!(std::fs::read(&live).unwrap(), b"unexpected-live-file");
}
