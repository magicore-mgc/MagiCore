//! Tests rejection of invalid managed-bin inputs before OS effects.
//! Kiểm tra từ chối bin không hợp lệ trước khi có tác động lên OS.

use super::super::ensure_bin_on_path;

#[test]
fn regular_file_is_rejected_before_os_path_update() {
    let home = tempfile::tempdir().unwrap();
    let bin = home.path().join("not-a-directory");
    std::fs::write(&bin, b"file").unwrap();

    let error = ensure_bin_on_path(home.path(), "/bin/zsh", &bin)
        .expect_err("a regular file must not be accepted as the managed bin directory");
    assert!(error.to_string().contains("must be a real directory"));
}
