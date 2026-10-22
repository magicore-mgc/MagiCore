//! Keyring save transaction tests — Test transaction save keyring.
//! Save tests exercise the production path policy.
//! Test save đi qua policy production.

use super::{KeyPair, Keyring};
use std::fs;
use tempfile::{Builder, TempDir};

fn keyring_test_dir() -> TempDir {
    let home = dirs::home_dir().expect("home directory is required for keyring save tests");
    let home = home.canonicalize().unwrap_or(home);
    Builder::new()
        .prefix("mgc-keyring-save-test-")
        .tempdir_in(home)
        .expect("create keyring test directory under home")
}

#[test]
fn save_replaces_existing_keyring_and_rotates_backup_portably() {
    let directory = keyring_test_dir();
    let path = directory.path().join("keyring.json");
    let backup = path.with_extension("json.bak");
    let mut original = Keyring::new();
    original.add_key(KeyPair::generate().expect("test operation must succeed"));
    let original_id = original.default_key_id.clone();
    original.save(&path).expect("test operation must succeed");

    let mut replacement = Keyring::new();
    replacement.add_key(KeyPair::generate().expect("test operation must succeed"));
    replacement
        .save(&path)
        .expect("test operation must succeed");

    assert_eq!(
        Keyring::load(&path)
            .expect("test operation must succeed")
            .default_key_id,
        replacement.default_key_id
    );
    assert_eq!(
        Keyring::load(&backup)
            .expect("test operation must succeed")
            .default_key_id,
        original_id
    );
}

#[test]
#[cfg(unix)]
fn save_publishes_private_keyring_and_backup_without_leaking_temp_files() {
    use std::os::unix::fs::PermissionsExt;

    let directory = keyring_test_dir();
    let path = directory.path().join("keyring.json");
    let backup = path.with_extension("json.bak");
    let mut original = Keyring::new();
    original.add_key(KeyPair::generate().expect("test operation must succeed"));
    original.save(&path).expect("test operation must succeed");
    let original_id = original.default_key_id.clone();

    let mut replacement = Keyring::new();
    replacement.add_key(KeyPair::generate().expect("test operation must succeed"));
    replacement
        .save(&path)
        .expect("test operation must succeed");

    assert_eq!(
        Keyring::load(&path)
            .expect("test operation must succeed")
            .default_key_id,
        replacement.default_key_id
    );
    assert_eq!(
        Keyring::load(&backup)
            .expect("test operation must succeed")
            .default_key_id,
        original_id
    );
    for candidate in [path, backup] {
        assert_eq!(
            fs::metadata(candidate)
                .expect("test operation must succeed")
                .permissions()
                .mode()
                & 0o777,
            0o600
        );
    }
    assert!(
        directory
            .path()
            .read_dir()
            .expect("test operation must succeed")
            .all(|entry| {
                !entry
                    .expect("test operation must succeed")
                    .file_name()
                    .to_string_lossy()
                    .ends_with(".tmp")
            })
    );
}

#[test]
#[cfg(unix)]
fn save_rejects_insecure_existing_permissions_without_mutating_keyring() {
    use std::os::unix::fs::PermissionsExt;

    let directory = keyring_test_dir();
    let path = directory.path().join("keyring.json");
    let mut first = Keyring::new();
    first.add_key(KeyPair::generate().expect("test operation must succeed"));
    first.save(&path).expect("test operation must succeed");
    fs::set_permissions(&path, fs::Permissions::from_mode(0o644))
        .expect("test operation must succeed");

    let mut replacement = Keyring::new();
    replacement.add_key(KeyPair::generate().expect("test operation must succeed"));
    let original = fs::read(&path).expect("test operation must succeed");
    assert!(replacement.save(&path).is_err());

    assert_eq!(
        fs::read(&path).expect("test operation must succeed"),
        original
    );
    assert_eq!(
        fs::metadata(&path)
            .expect("test operation must succeed")
            .permissions()
            .mode()
            & 0o777,
        0o644
    );
}

#[test]
#[cfg(unix)]
fn save_refuses_keyring_symlink_without_writing_through_it() {
    use std::os::unix::fs::symlink;

    let directory = keyring_test_dir();
    let target = directory.path().join("target.json");
    let path = directory.path().join("keyring.json");
    fs::write(&target, b"do not overwrite").expect("test operation must succeed");
    symlink(&target, &path).expect("test operation must succeed");
    let mut keyring = Keyring::new();
    keyring.add_key(KeyPair::generate().expect("test operation must succeed"));

    assert!(keyring.save(&path).is_err());
    assert_eq!(
        fs::read(&target).expect("test operation must succeed"),
        b"do not overwrite"
    );
    assert!(
        fs::symlink_metadata(&path)
            .expect("test operation must succeed")
            .file_type()
            .is_symlink()
    );
}

#[test]
#[cfg(unix)]
fn save_refuses_backup_symlink_without_writing_through_it() {
    use std::os::unix::fs::PermissionsExt;
    use std::os::unix::fs::symlink;

    let directory = keyring_test_dir();
    let path = directory.path().join("keyring.json");
    let target = directory.path().join("outside.json");
    let backup = path.with_extension("json.bak");
    fs::write(&target, b"do not overwrite").expect("test operation must succeed");
    let mut original = Keyring::new();
    original.add_key(KeyPair::generate().expect("test operation must succeed"));
    original.save(&path).expect("test operation must succeed");
    fs::set_permissions(&path, fs::Permissions::from_mode(0o600))
        .expect("test operation must succeed");
    symlink(&target, &backup).expect("test operation must succeed");

    let mut replacement = Keyring::new();
    replacement.add_key(KeyPair::generate().expect("test operation must succeed"));
    assert!(replacement.save(&path).is_err());
    assert_eq!(
        fs::read(&target).expect("test operation must succeed"),
        b"do not overwrite"
    );
    assert!(
        fs::symlink_metadata(&backup)
            .expect("test operation must succeed")
            .file_type()
            .is_symlink()
    );
    assert_eq!(
        Keyring::load(&path)
            .expect("test operation must succeed")
            .default_key_id,
        original.default_key_id
    );
}
