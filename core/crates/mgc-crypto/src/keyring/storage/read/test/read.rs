//! Secure keyring read tests — Test đọc keyring bảo mật.
//! Tests use temporary files and never expose key material — Dùng file tạm và không lộ key.

use super::read::read_keyring_contents;
use std::fs;

#[test]
fn reads_regular_file_bytes_as_text() {
    let directory = tempfile::tempdir().expect("test operation must succeed");
    let path = directory.path().join("keyring.json");
    fs::write(&path, b"{}\n").expect("test operation must succeed");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&path, fs::Permissions::from_mode(0o600))
            .expect("test operation must succeed");
    }
    assert_eq!(
        read_keyring_contents(&path).expect("test operation must succeed"),
        "{}\n"
    );
}

#[test]
fn rejects_a_directory_as_keyring_file() {
    let directory = tempfile::tempdir().expect("test operation must succeed");
    assert!(read_keyring_contents(directory.path()).is_err());
}
