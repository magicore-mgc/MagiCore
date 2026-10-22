//! Export Permissions source regression test.
//! Test hồi quy source cho owner export permissions.

use std::fs;

#[test]
fn applies_requested_exported_permissions() {
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("exported");
    fs::write(&path, b"mode").unwrap();

    super::super::export_permissions::set_exported_permissions(&path, false).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        assert_eq!(
            fs::metadata(&path).unwrap().permissions().mode() & 0o777,
            0o644
        );
    }

    super::super::export_permissions::set_exported_permissions(&path, true).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        assert_eq!(
            fs::metadata(&path).unwrap().permissions().mode() & 0o777,
            0o755
        );
    }
}
