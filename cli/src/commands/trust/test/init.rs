//! Tests for `mgc trust init` keyring creation.
//! Test cho tạo keyring của `mgc trust init`.
use super::execute_at;
use mgc_crypto::keyring::{KeyPair, Keyring};
use std::fs;

#[test]
fn invalid_keyring_and_valid_backup_are_preserved_and_reported() {
    let directory = tempfile::tempdir().unwrap();
    let keyring_path = directory.path().join("keyring.json");
    let backup_path = directory.path().join("keyring.json.bak");
    fs::write(&keyring_path, b"not-json").unwrap();

    let mut backup = Keyring::new();
    backup.add_key(KeyPair::generate().unwrap());
    write_keyring_fixture(&backup_path, &backup);
    let original = fs::read(&keyring_path).unwrap();
    let original_backup = fs::read(&backup_path).unwrap();

    let error = execute_at(&keyring_path, false).unwrap_err().to_string();

    assert!(error.contains("valid backup exists"), "{error}");
    assert!(error.contains("No files were changed"), "{error}");
    assert_eq!(fs::read(&keyring_path).unwrap(), original);
    assert_eq!(fs::read(&backup_path).unwrap(), original_backup);
}

#[test]
fn invalid_keyring_without_backup_returns_error_instead_of_success() {
    let directory = tempfile::tempdir().unwrap();
    let keyring_path = directory.path().join("keyring.json");
    fs::write(&keyring_path, b"not-json").unwrap();

    let error = execute_at(&keyring_path, false).unwrap_err().to_string();

    assert!(error.contains("No backup exists"), "{error}");
    assert!(error.contains("No files were changed"), "{error}");
    assert_eq!(fs::read(&keyring_path).unwrap(), b"not-json");
}

#[test]
#[cfg(unix)]
fn dangling_keyring_symlink_is_rejected_without_creating_its_target() {
    use std::os::unix::fs::symlink;

    let directory = tempfile::tempdir().unwrap();
    let keyring_path = directory.path().join("keyring.json");
    let target_path = directory.path().join("outside.json");
    symlink(&target_path, &keyring_path).unwrap();

    let error = execute_at(&keyring_path, false).unwrap_err().to_string();

    assert!(
        error.contains("must be a regular non-symlink file"),
        "{error}"
    );
    assert!(
        !target_path.exists(),
        "init must not create a dangling-link target"
    );
    assert!(
        fs::symlink_metadata(&keyring_path)
            .unwrap()
            .file_type()
            .is_symlink()
    );
}

fn write_keyring_fixture(path: &std::path::Path, keyring: &Keyring) {
    let json = serde_json::to_vec_pretty(keyring).unwrap();
    fs::write(path, json).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o600)).unwrap();
    }
}
