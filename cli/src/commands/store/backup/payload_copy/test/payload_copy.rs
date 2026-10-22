//! Tests for no-follow backup payload copy — kiểm tra copy payload no-follow.

use super::super::copy_backup_payload;
use std::fs;

#[test]
fn copies_database_and_reports_its_size() {
    let directory = tempfile::tempdir().unwrap();
    let source = directory.path().join("source.db");
    let backup = directory.path().join("backup");
    fs::create_dir(&backup).unwrap();
    fs::write(&source, b"db").unwrap();

    let payload = copy_backup_payload(&backup, Some(&source), None).unwrap();

    assert_eq!(payload.database_bytes, 2);
    assert_eq!(fs::read(backup.join("store.db")).unwrap(), b"db");
}
