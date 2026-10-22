//! Tests that directory creation rejects symlinked path components.
//! Kiểm tra tạo thư mục từ chối thành phần symlink.

use super::super::ensure_directory_chain;

#[cfg(unix)]
#[test]
fn symlink_component_is_rejected() {
    let home = tempfile::tempdir().unwrap();
    let target = tempfile::tempdir().unwrap();
    let link = home.path().join("linked-config");
    std::os::unix::fs::symlink(target.path(), &link).unwrap();

    let error = ensure_directory_chain(home.path(), &link.join("fish"))
        .expect_err("a symlink component must be rejected");
    assert!(error.to_string().contains("is unsafe"));
    assert!(!target.path().join("fish").exists());
}
