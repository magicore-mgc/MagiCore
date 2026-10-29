//! Limit-constant contract tests — test contract hằng giới hạn.
//! The numbers come from external specifications; these tests record that fact so
//! a later "cleanup" cannot quietly change one and break interop.
//! (Con số đến từ spec ngoài; test ghi lại điều đó để lần "dọn dẹp" sau không
//! âm thầm đổi và phá tương thích.)

use super::{
    HTTP_SERVER_ERROR_MIN, HTTP_TOO_MANY_REQUESTS, LINUX_INTERFACE_NAME_MAX, NPM_PACKAGE_NAME_MAX,
    OCI_TAG_MAX, PATH_SEGMENT_MAX,
};

#[test]
fn the_limits_match_their_specifications() {
    // npm: package name length limit enforced by the registry.
    assert_eq!(NPM_PACKAGE_NAME_MAX, 214);
    // POSIX-filesystem segment limit shared by ext4, APFS and NTFS.
    assert_eq!(PATH_SEGMENT_MAX, 255);
    // OCI: tag grammar allows up to 128 characters total.
    assert_eq!(OCI_TAG_MAX, 128);
    // Linux: IFNAMSIZ is 16; this constant is the documented 256-byte buffer.
    assert_eq!(LINUX_INTERFACE_NAME_MAX, 256);
    // HTTP: the two multi-purpose statuses used for retry decisions.
    assert_eq!(HTTP_TOO_MANY_REQUESTS, 429);
    assert_eq!(HTTP_SERVER_ERROR_MIN, 500);
}

/// The two retry statuses must not overlap, so a caller matching either one is
/// unambiguous.
/// (Hai mã retry không được trùng để caller khớp một trong hai là rõ ràng.)
#[test]
fn the_retry_statuses_are_distinct() {
    assert_ne!(HTTP_SERVER_ERROR_MIN, HTTP_TOO_MANY_REQUESTS);
}

/// A tag is appended to a name in `name:tag`, so the tag budget must be smaller
/// than a full path segment.
/// (Tag nối sau tên trong `name:tag`, nên ngân sách tag phải nhỏ hơn một segment.)
#[test]
fn a_tag_fits_inside_a_path_segment() {
    assert_ne!(OCI_TAG_MAX, PATH_SEGMENT_MAX);
}
