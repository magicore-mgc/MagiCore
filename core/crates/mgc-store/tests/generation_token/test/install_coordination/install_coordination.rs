//! Install guard cleanup and project-scoped locking.
//! Dọn staging bằng guard và khóa install theo project.

use crate::generation_token::support::{PROJ_A, PROJ_B};
use mgc_store::Database;

pub(crate) fn generation_guard_aborts_staging_on_drop() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("store.db");
    let db = Database::open(&db_path).unwrap();

    let (token, live_during) = {
        let guard = db.begin_cas_generation_guarded(&db_path, PROJ_A).unwrap();
        db.cas_claim(PROJ_A, guard.generation(), "hash-g").unwrap();
        let live = db.list_cas_live_refs().unwrap();
        (guard.generation(), live)
    };
    // Guard dropped without disarm → the token is aborted, its claims
    // retired. This is the leak-proof contract for failed installs.
    // (Guard drop không disarm → token bị hủy, claim nghỉ hưu. Đây là
    // hợp đồng không-rò cho install fail.)
    assert_eq!(
        live_during,
        vec!["hash-g".to_string()],
        "claims are live while the guard holds the token"
    );
    let live_after = db.list_cas_live_refs().unwrap();
    assert!(
        live_after.is_empty(),
        "P0 guard: drop must abort the staging token, live = {live_after:?}, token = {token}"
    );
}

pub(crate) fn generation_guard_disarm_keeps_promoted_claims() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("store.db");
    let db = Database::open(&db_path).unwrap();

    let guard = db.begin_cas_generation_guarded(&db_path, PROJ_A).unwrap();
    let token = guard.generation();
    db.cas_claim(PROJ_A, token, "hash-gp").unwrap();
    db.promote_cas_generation(PROJ_A, token).unwrap();
    // Disarm AFTER promote — drop must NOT abort a promoted token.
    // (Disarm SAU promote — drop KHÔNG được hủy token đã promote.)
    guard.disarm();
    drop(db);

    let db = Database::open(&db_path).unwrap();
    assert_eq!(
        db.list_cas_live_refs().unwrap(),
        vec!["hash-gp".to_string()],
        "disarmed guard must leave the promoted claims live"
    );
}

pub(crate) fn project_install_lock_serializes_same_project() {
    let dir = tempfile::tempdir().unwrap();
    // Hermetic locks root (no global env mutation, no user-store touch)
    // — the public test seam of the lock.
    // (Root locks kín — đường test công khai của lock, không đổi env
    // toàn cục, không chạm store user.)
    let locks_root = dir.path().join("locks");
    let db = Database::open(&dir.path().join("store.db")).unwrap();
    let _ = &db; // the lock is store-level, not database-level

    let first = mgc_store::ProjectInstallLock::acquire_at(&locks_root, PROJ_A).unwrap();
    // A second lock on the SAME project must fail (try-lock, not block —
    // a blocking test would hang). A DIFFERENT project must still
    // acquire — the lock is per-project.
    // (Lock thứ hai trên CÙNG project phải fail (try-lock, không chặn
    // — chặn thì test treo). Project KHÁC vẫn phải lấy được — lock theo
    // từng project.)
    let second = mgc_store::ProjectInstallLock::acquire_at(&locks_root, PROJ_A);
    assert!(
        second.is_err(),
        "a second install of the same project must be refused, not raced"
    );
    assert!(
        mgc_store::ProjectInstallLock::acquire_at(&locks_root, PROJ_B).is_ok(),
        "a different project must acquire its own lock concurrently"
    );
    drop(first);
    // After release, the next install acquires.
    // (Sau khi nhả, install kế tiếp lấy được.)
    assert!(mgc_store::ProjectInstallLock::acquire_at(&locks_root, PROJ_A).is_ok());
}
