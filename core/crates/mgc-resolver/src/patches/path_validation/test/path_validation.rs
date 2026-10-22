//! Path validation boundary tests; no filesystem or network required.
//! Test biên kiểm tra đường dẫn; không cần filesystem hay mạng.

#![allow(clippy::unwrap_used)]

use super::validate_patch_relative_path;

#[test]
fn accepts_nested_relative_path() {
    assert!(validate_patch_relative_path("src/lib.rs").is_ok());
}

#[test]
fn rejects_empty_and_windows_separator_paths() {
    assert!(validate_patch_relative_path("").is_err());
    assert!(validate_patch_relative_path("..\\secret").is_err());
    assert!(validate_patch_relative_path("bad\0path").is_err());
}

#[test]
fn rejects_parent_and_root_components() {
    assert!(validate_patch_relative_path("../secret").is_err());
    assert!(validate_patch_relative_path("/etc/passwd").is_err());
}
