//! Verify traversal rejection and exact regular-file enumeration.
//! Kiểm tra từ chối traversal và liệt kê file thường chính xác.

use super::super::{collect_regular_files, validate_record_path, verified_site_file_path};
use std::collections::BTreeSet;

#[test]
fn record_paths_reject_absolute_and_traversal_components() {
    for path in [
        "",
        "/absolute/file",
        r"\absolute\file",
        "pkg\\file.py",
        "pkg:name/file.py",
        "pkg//file.py",
        "pkg/./file.py",
        "pkg/../file.py",
        "pkg/file\0.py",
    ] {
        assert!(validate_record_path(path).is_err(), "accepted {path:?}");
    }
    assert!(validate_record_path("pkg/module.py").is_ok());
}

#[test]
fn record_paths_enumerate_nested_files_with_posix_names() {
    let site = tempfile::tempdir().expect("test operation must succeed");
    std::fs::create_dir_all(site.path().join("pkg/sub")).expect("test operation must succeed");
    std::fs::write(site.path().join("pkg/sub/module.py"), b"value")
        .expect("test operation must succeed");
    let file = verified_site_file_path(site.path(), "pkg/sub/module.py")
        .expect("test operation must succeed");
    assert_eq!(file, site.path().join("pkg/sub/module.py"));

    let mut actual = BTreeSet::new();
    collect_regular_files(site.path(), site.path(), &mut actual)
        .expect("test operation must succeed");
    assert_eq!(actual, BTreeSet::from(["pkg/sub/module.py".to_string()]));
}

#[cfg(target_os = "linux")]
#[test]
fn record_paths_reject_non_utf8_installed_names_without_lossy_aliases() {
    use std::os::unix::ffi::OsStringExt;

    let site = tempfile::tempdir().expect("test operation must succeed");
    let invalid_name = std::ffi::OsString::from_vec(vec![b'm', 0xff, b'.', b'p', b'y']);
    std::fs::write(site.path().join(invalid_name), b"payload")
        .expect("test operation must succeed");

    let error = collect_regular_files(site.path(), site.path(), &mut BTreeSet::new())
        .expect_err("test operation must fail");
    assert!(error.to_string().contains("non-UTF-8 path"));
}
