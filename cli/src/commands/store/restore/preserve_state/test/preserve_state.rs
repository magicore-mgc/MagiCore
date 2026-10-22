//! Pre-restore state preservation cases.
//! Ca bảo toàn state trước restore.

use super::super::preserve_current_state;
use std::fs;

#[test]
fn preserves_existing_database_and_virtual_store() {
    let temp = tempfile::tempdir().unwrap();
    let database = temp.path().join("project/store.db");
    let virtual_store = temp.path().join("project/node_modules/.magicore");
    fs::create_dir_all(database.parent().unwrap()).unwrap();
    fs::create_dir_all(&virtual_store).unwrap();
    fs::write(&database, b"database bytes").unwrap();
    fs::write(virtual_store.join("package.bin"), b"package bytes").unwrap();
    let safety = temp.path().join("safety");

    preserve_current_state(Some(&database), Some(&virtual_store), &safety).unwrap();

    assert_eq!(
        fs::read(safety.join("store.db")).unwrap(),
        b"database bytes"
    );
    assert_eq!(
        fs::read(safety.join("vstore/package.bin")).unwrap(),
        b"package bytes"
    );
}

#[test]
fn absent_components_do_not_create_placeholder_entries() {
    let temp = tempfile::tempdir().unwrap();
    let safety = temp.path().join("safety");
    preserve_current_state(None, None, &safety).unwrap();

    assert!(!safety.join("store.db").exists());
    assert!(!safety.join("vstore").exists());
}

#[test]
fn refuses_to_merge_current_state_into_a_non_empty_safety_backup() {
    let temp = tempfile::tempdir().unwrap();
    let database = temp.path().join("project/store.db");
    let safety = temp.path().join("safety");
    fs::create_dir_all(database.parent().unwrap()).unwrap();
    fs::write(&database, b"current database").unwrap();
    fs::create_dir(&safety).unwrap();
    fs::write(safety.join("stale.bin"), b"old safety snapshot").unwrap();

    assert!(preserve_current_state(Some(&database), None, &safety).is_err());
    assert_eq!(
        fs::read(safety.join("stale.bin")).unwrap(),
        b"old safety snapshot"
    );
    assert!(!safety.join("store.db").exists());
}
