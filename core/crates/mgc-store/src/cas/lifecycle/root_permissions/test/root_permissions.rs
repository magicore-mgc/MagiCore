//! CAS root permission tests.
//! Test permission root CAS.

#[cfg(unix)]
#[test]
fn existing_root_is_tightened_to_owner_only() {
    use std::os::unix::fs::PermissionsExt;

    let root = tempfile::tempdir().unwrap();
    std::fs::set_permissions(root.path(), std::fs::Permissions::from_mode(0o777)).unwrap();
    super::super::set_cas_root_permissions(root.path()).unwrap();
    let mode = std::fs::metadata(root.path()).unwrap().permissions().mode() & 0o777;
    assert_eq!(mode, 0o700);
}

#[cfg(not(unix))]
#[test]
fn existing_root_is_accepted_on_non_unix() {
    let root = tempfile::tempdir().unwrap();
    super::super::set_cas_root_permissions(root.path()).unwrap();
}
