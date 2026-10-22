//! Temporary project fixture for executor integration tests.
//! Fixture project tạm cho integration test executor.

// RAII removes each fixture on success and panic, preventing temp leaks.
// RAII dọn fixture cả khi pass lẫn panic để test không tích lũy dữ liệu tạm.
pub(in crate::run) fn tmp_dir() -> tempfile::TempDir {
    tempfile::Builder::new()
        .prefix("mgc-exec-test-")
        .tempdir()
        .expect("create unique executor test directory")
}
