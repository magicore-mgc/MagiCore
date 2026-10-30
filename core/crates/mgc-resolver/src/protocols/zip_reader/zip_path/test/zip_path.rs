//! Covers traversal rejection and Windows-unsafe entry names.
//! Kiểm tra từ chối traversal và tên entry không an toàn trên Windows.

use std::path::Path;

#[test]
fn rejects_parent_and_empty_zip_paths() {
    assert!(super::zip_path::sanitize_zip_path("../escape").is_err());
    assert!(super::zip_path::sanitize_zip_path("").is_err());
    assert!(super::zip_path::sanitize_zip_path("pkg/../../escape").is_err());
}

#[test]
fn removes_current_directory_components() {
    let path = super::zip_path::sanitize_zip_path("./pkg/./file")
        .expect("ordinary path with . components must sanitize");
    assert_eq!(path, Path::new("pkg/file"));
}

#[test]
fn rejects_windows_unsafe_zip_paths() {
    // NTFS ADS, trailing dot/space, and reserved device names map to a different
    // file on Windows — all must be rejected.
    // NTFS ADS, dấu chấm/khoảng trắng cuối, và tên thiết bị dành riêng trỏ tới
    // file khác trên Windows — đều phải bị từ chối.
    for unsafe_path in [
        "pkg/file.txt:ads",
        "pkg/file:stream",
        "pkg/file.",
        "pkg/file ",
        "CON",
        "pkg/nul.txt",
        "pkg/COM1",
        "pkg/LPT9.log",
    ] {
        assert!(
            super::zip_path::sanitize_zip_path(unsafe_path).is_err(),
            "expected {unsafe_path:?} to be rejected"
        );
    }
}

#[test]
fn accepts_ordinary_names_that_look_windows_special() {
    // "console.log" and "com10" are ordinary names, not reserved devices.
    // "console.log" và "com10" là tên thường, không phải thiết bị dành riêng.
    for safe_path in ["pkg/console.log", "pkg/com10.txt", "pkg/nullify.txt"] {
        assert!(
            super::zip_path::sanitize_zip_path(safe_path).is_ok(),
            "expected {safe_path:?} to be accepted"
        );
    }
}
