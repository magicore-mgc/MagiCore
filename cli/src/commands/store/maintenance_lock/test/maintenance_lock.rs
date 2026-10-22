//! Maintenance commands and Web install must contend on the same lock file.
//! Lệnh bảo trì và Web install phải tranh chấp đúng cùng một file lock.

use crate::commands::store::acquire_maintenance_lock;

#[test]
fn maintenance_lock_uses_web_layout_root_and_locks_directory() {
    let temp = tempfile::tempdir().unwrap();
    let project_cache = temp.path().join(".magicore/cache/web");
    let layout = mgc_store::Layout::new(project_cache.clone());
    let maintenance_lock = acquire_maintenance_lock(temp.path()).unwrap();

    let second_lock = mgc_store::ProjectInstallLock::acquire_at(
        &layout.locks_dir(),
        &layout.root().to_string_lossy(),
    );

    assert!(second_lock.is_err());
    drop(maintenance_lock);
    assert!(
        mgc_store::ProjectInstallLock::acquire_at(
            &layout.locks_dir(),
            &layout.root().to_string_lossy(),
        )
        .is_ok()
    );
}
