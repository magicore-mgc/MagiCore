//! CAS directory creation tests.
//! Test tạo thư mục CAS.

#[test]
fn required_content_and_compiled_directories_are_created() {
    let root = tempfile::tempdir().unwrap();
    super::super::ensure_cas_dirs(root.path()).unwrap();
    assert!(root.path().join("files/blake3").is_dir());
    assert!(root.path().join("compiled/blake3").is_dir());
}
