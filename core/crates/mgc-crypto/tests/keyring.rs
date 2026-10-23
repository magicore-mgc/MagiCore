//! Tests for keyring module
//! Tests cho module keyring

use mgc_crypto::keyring::{KeyPair, Keyring};
use std::fs;

fn keyring_test_dir() -> tempfile::TempDir {
    let home =
        dirs::home_dir().expect("home directory is required for production keyring save tests");
    let home = home.canonicalize().unwrap_or(home);
    tempfile::Builder::new()
        .prefix("mgc-keyring-test-")
        .tempdir_in(home)
        .expect("create keyring test directory under home")
}

#[test]
fn test_keyring_add_key() {
    let mut keyring = Keyring::new();
    let key_pair = KeyPair::generate().expect("test operation must succeed");
    let key_id = key_pair.key_id.clone();

    keyring.add_key(key_pair);
    assert_eq!(keyring.keys.len(), 1);
    assert_eq!(keyring.default_key_id, Some(key_id.clone()));

    let retrieved = keyring
        .get_key(&key_id)
        .expect("test operation must succeed");
    assert_eq!(retrieved.key_id, key_id);
}

#[test]
fn test_keyring_save_load() {
    let dir = keyring_test_dir();
    let path = dir.path().join("keyring.json");

    let mut keyring = Keyring::new();
    let key_pair = KeyPair::generate().expect("test operation must succeed");
    keyring.add_key(key_pair);

    keyring.save(&path).expect("test operation must succeed");
    let loaded = Keyring::load(&path).expect("test operation must succeed");

    assert_eq!(keyring.keys.len(), loaded.keys.len());
    assert_eq!(keyring.default_key_id, loaded.default_key_id);
}

#[test]
fn test_keyring_default_key() {
    let mut keyring = Keyring::new();
    let key1 = KeyPair::generate().expect("test operation must succeed");
    let key2 = KeyPair::generate().expect("test operation must succeed");

    keyring.add_key(key1.clone());
    keyring.add_key(key2.clone());

    let default = keyring.default_key().expect("test operation must succeed");
    assert_eq!(default.key_id, key1.key_id);

    keyring
        .set_default(&key2.key_id)
        .expect("test operation must succeed");
    let default = keyring.default_key().expect("test operation must succeed");
    assert_eq!(default.key_id, key2.key_id);
}

#[test]
#[cfg(unix)]
fn test_keyring_secure_permissions() {
    use std::os::unix::fs::PermissionsExt;

    let dir = keyring_test_dir();
    let path = dir.path().join("keyring.json");

    let mut keyring = Keyring::new();
    let key_pair = KeyPair::generate().expect("test operation must succeed");
    keyring.add_key(key_pair);

    keyring.save(&path).expect("test operation must succeed");

    let metadata = fs::metadata(&path).expect("test operation must succeed");
    let mode = metadata.permissions().mode();
    assert_eq!(mode & 0o777, 0o600); // Owner read/write only — Chỉ owner đọc/ghi
}

#[test]
#[cfg(unix)]
fn test_keyring_load_rejects_group_or_world_readable_private_keys() {
    use std::os::unix::fs::PermissionsExt;

    let dir = keyring_test_dir();
    let path = dir.path().join("keyring.json");
    let mut keyring = Keyring::new();
    keyring.add_key(KeyPair::generate().expect("test operation must succeed"));
    keyring.save(&path).expect("test operation must succeed");

    fs::set_permissions(&path, fs::Permissions::from_mode(0o644))
        .expect("test operation must succeed");

    let error = Keyring::load(&path).expect_err("test operation must fail");

    assert!(
        error.to_string().contains("permissions") && error.to_string().contains("group/world"),
        "unsafe private-key file must be rejected explicitly, got: {error}"
    );
}

#[test]
#[cfg(unix)]
fn test_keyring_load_rejects_symlink_path() {
    use std::os::unix::fs::symlink;

    let dir = keyring_test_dir();
    let target = dir.path().join("target.json");
    let link = dir.path().join("keyring.json");
    let mut keyring = Keyring::new();
    keyring.add_key(KeyPair::generate().expect("test operation must succeed"));
    keyring.save(&target).expect("test operation must succeed");
    symlink(&target, &link).expect("test operation must succeed");

    assert!(Keyring::load(&link).is_err());
    assert!(target.exists(), "load must not mutate the symlink target");
}

#[test]
#[cfg(unix)]
// SAFETY: `mkfifo` receives a valid NUL-terminated path pointer from `CString`
// (no interior NUL possible) with mode 0o600; return value is checked and the
// FIFO is created inside an isolated tempdir. Test-only helper, no production use.
// AN TOÀN: `mkfifo` nhận con trỏ path NUL-terminated hợp lệ từ `CString` (không
// thể có NUL giữa), mode 0o600; kiểm tra return và FIFO nằm trong tempdir cô lập.
// Helper chỉ dùng cho test, không dùng production.
#[allow(unsafe_code)]
fn test_keyring_load_rejects_fifo_without_blocking() {
    use std::ffi::CString;
    use std::os::unix::ffi::OsStrExt;
    use std::sync::mpsc;
    use std::time::Duration;

    let dir = keyring_test_dir();
    let path = dir.path().join("keyring.fifo");
    let c_path = CString::new(path.as_os_str().as_bytes()).expect("test operation must succeed");
    // SAFETY: test-only operation on process-local state or FFI with
    // caller-owned arguments valid for the call.
    // AN TOÀN: chỉ test thao tác state cục bộ process hoặc FFI với
    // đối số do caller sở hữu, hợp lệ trong lời gọi.
    let mkfifo_result = unsafe { libc::mkfifo(c_path.as_ptr(), 0o600) };
    assert_eq!(
        mkfifo_result,
        0,
        "mkfifo failed: {}",
        std::io::Error::last_os_error()
    );

    let (sender, receiver) = mpsc::channel();
    std::thread::spawn(move || {
        let result = Keyring::load(&path);
        let _ = sender.send(result.map(|_| ()).map_err(|error| error.to_string()));
    });

    let result = receiver
        .recv_timeout(Duration::from_millis(500))
        .expect("loading a FIFO must not hang before rejecting the non-regular file");
    assert!(result.is_err(), "FIFO must be rejected, got {result:?}");
}
