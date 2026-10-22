use super::super::super::test_support::temp_dir_with_npmrc;
use super::super::*;
use std::collections::BTreeMap;
use std::fs;

#[test]
fn set_npmrc_creates_and_updates_key() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join(".npmrc");
    // Lần đầu: tạo mới
    set_npmrc(&path, "registry", "https://example.com").unwrap();
    let content = fs::read_to_string(&path).unwrap();
    assert!(content.contains("registry=https://example.com"));
    // Lần hai: ghi đè
    set_npmrc(&path, "registry", "https://other.com").unwrap();
    let content2 = fs::read_to_string(&path).unwrap();
    assert!(content2.contains("registry=https://other.com"));
    assert!(!content2.contains("example.com"));
}

#[cfg(unix)]
#[test]
fn new_npmrc_is_private_and_existing_mode_is_preserved() {
    use std::os::unix::fs::PermissionsExt;

    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join(".npmrc");
    set_npmrc(&path, "registry", "https://example.test").unwrap();
    assert_eq!(
        fs::metadata(&path).unwrap().permissions().mode() & 0o777,
        0o600
    );

    fs::set_permissions(&path, fs::Permissions::from_mode(0o640)).unwrap();
    set_npmrc(&path, "registry", "https://next.example.test").unwrap();
    assert_eq!(
        fs::metadata(&path).unwrap().permissions().mode() & 0o777,
        0o640
    );
}

#[cfg(unix)]
#[test]
fn writing_npmrc_credentials_tightens_permissive_existing_mode() {
    use std::os::unix::fs::PermissionsExt;

    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join(".npmrc");
    fs::write(&path, "registry=https://registry.example\n").unwrap();
    fs::set_permissions(&path, fs::Permissions::from_mode(0o644)).unwrap();

    set_npmrc(&path, "//registry.example/:_authToken", "sensitive-token").unwrap();

    assert_eq!(
        fs::metadata(&path).unwrap().permissions().mode() & 0o777,
        0o600,
        "a config containing credentials must not retain group/world access"
    );
    fs::set_permissions(&path, fs::Permissions::from_mode(0o644)).unwrap();
    set_npmrc(&path, "registry", "https://next.example").unwrap();
    assert_eq!(
        fs::metadata(&path).unwrap().permissions().mode() & 0o777,
        0o600,
        "updating a file that already contains credentials must also tighten it"
    );
    assert!(
        fs::read_to_string(path)
            .unwrap()
            .contains("sensitive-token")
    );
}

#[test]
fn delete_npmrc_removes_key() {
    let dir = temp_dir_with_npmrc("registry=https://example.com\nfoo=bar\n");
    let path = dir.path().join(".npmrc");
    delete_npmrc(&path, "registry").unwrap();
    let content = fs::read_to_string(&path).unwrap();
    assert!(!content.contains("registry="));
    assert!(content.contains("foo=bar"));
}

#[test]
fn merge_file_combines_keys() {
    let dir = temp_dir_with_npmrc("key1=val1\nkey2=val2\n");
    let mut map = BTreeMap::new();
    merge_file(&mut map, &dir.path().join(".npmrc"));
    assert_eq!(map.get("key1").map(|s| s.as_str()), Some("val1"));
    assert_eq!(map.get("key2").map(|s| s.as_str()), Some("val2"));
}

#[test]
fn merge_file_no_panic_on_missing_file() {
    let mut map = BTreeMap::new();
    // Should not panic on missing file
    merge_file(&mut map, std::path::Path::new("/nonexistent/path/.npmrc"));
    assert!(map.is_empty());
}
