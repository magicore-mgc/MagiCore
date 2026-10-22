//! Platform executable metadata contracts.
//! Contract metadata executable theo nền tảng.

use super::super::{metadata_is_executable, mode_is_executable};

#[cfg(unix)]
#[test]
fn unix_mode_detects_any_execute_bit() {
    assert!(mode_is_executable(0o100));
    assert!(!mode_is_executable(0o644));
}

#[cfg(unix)]
#[test]
fn unix_file_metadata_detects_execute_permissions() {
    use std::os::unix::fs::PermissionsExt;

    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("bin");
    std::fs::write(&path, b"binary").unwrap();
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755)).unwrap();

    assert!(metadata_is_executable(&std::fs::metadata(path).unwrap()));
}

#[cfg(not(unix))]
#[test]
fn non_unix_mode_is_conservatively_non_executable() {
    assert!(!mode_is_executable(0o777));
}

#[cfg(not(unix))]
#[test]
fn non_unix_file_metadata_is_conservatively_non_executable() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("bin");
    std::fs::write(&path, b"binary").unwrap();

    assert!(!metadata_is_executable(&std::fs::metadata(path).unwrap()));
}
