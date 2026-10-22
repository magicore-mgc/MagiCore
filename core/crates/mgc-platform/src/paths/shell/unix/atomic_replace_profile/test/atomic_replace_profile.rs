//! Tests complete content replacement and temporary-file cleanup.
//! Kiểm tra thay nội dung hoàn chỉnh và dọn file tạm.

use super::super::atomic_replace_profile;

#[cfg(unix)]
#[test]
fn replacement_is_complete_and_leaves_no_temporary_file() {
    let directory = tempfile::tempdir().unwrap();
    let profile = directory.path().join(".profile");
    std::fs::write(&profile, "old profile\n").unwrap();
    let prior = std::fs::metadata(&profile).unwrap();

    atomic_replace_profile(&profile, b"new profile\n", Some(&prior)).unwrap();

    assert_eq!(std::fs::read_to_string(&profile).unwrap(), "new profile\n");
    assert_eq!(std::fs::read_dir(directory.path()).unwrap().count(), 1);
}
