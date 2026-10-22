//! TreeBackup take, commit, rollback, and Drop behavior tests.
//! Test take, commit, rollback và Drop của TreeBackup.

use std::path::PathBuf;

fn tmp(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("mgc-tree-backup-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn tree_backup_take_rollback_commit_cycle() {
    use crate::install::TreeBackup;
    let dir = tmp("guard");
    let live = dir.join("node_modules");
    std::fs::create_dir_all(live.join("pkg")).unwrap();
    std::fs::write(live.join("pkg").join("a.txt"), b"orig").unwrap();

    // take(): live moved aside atomically, fresh live recreated.
    let mut guard = TreeBackup::take(&live).unwrap();
    assert!(live.is_dir(), "fresh live dir recreated");
    assert!(!live.join("pkg").exists(), "fresh live starts empty");
    std::fs::write(live.join("new.txt"), b"partial").unwrap();

    // rollback(): pre-install tree back, byte-identical.
    guard.rollback().unwrap();
    assert_eq!(
        std::fs::read(live.join("pkg").join("a.txt")).unwrap(),
        b"orig"
    );
    assert!(!live.join("new.txt").exists());

    // commit(): backup gone, Drop becomes a no-op.
    let guard2 = TreeBackup::take(&live).unwrap();
    std::fs::write(live.join("b.txt"), b"new").unwrap();
    guard2.commit();
    assert!(live.join("b.txt").is_file(), "committed tree stands");
    assert!(!live.join("pkg").exists(), "old tree retired with backup");
}

#[test]
fn tree_backup_drop_auto_restores() {
    use crate::install::TreeBackup;
    let dir = tmp("guard-drop");
    let live = dir.join("node_modules");
    std::fs::create_dir_all(&live).unwrap();
    std::fs::write(live.join("keep.txt"), b"k").unwrap();
    {
        let _guard = TreeBackup::take(&live).unwrap();
        std::fs::write(live.join("junk.txt"), b"j").unwrap();
        // No commit, no rollback: Drop must restore.
    }
    assert_eq!(std::fs::read(live.join("keep.txt")).unwrap(), b"k");
    assert!(
        !live.join("junk.txt").exists(),
        "Drop restored pre-install tree"
    );
}

/// REVIEW: an EMPTY live dir carries no state — take() must skip it,
/// or a later rollback would restore emptiness over a fresh tree.
/// Dir trống không backup (rollback sau đó sẽ xóa cây mới).
#[test]
fn tree_backup_skips_empty_live_dir() {
    use crate::install::TreeBackup;
    let dir = tmp("guard-empty");
    let live = dir.join("node_modules");
    std::fs::create_dir_all(&live).unwrap();
    let guard = TreeBackup::take(&live).unwrap();
    // No backup taken: Drop must be a silent no-op, live untouched.
    std::fs::write(live.join("new.txt"), b"n").unwrap();
    drop(guard);
    assert_eq!(std::fs::read(live.join("new.txt")).unwrap(), b"n");
}
