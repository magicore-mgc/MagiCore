//! Verifies file opening delegates to the bounded reader implementation.
//! Xác minh mở file chuyển tiếp vào implementation reader có giới hạn.

use super::super::super::test_support::gzip_tar_file;

#[test]
fn extracts_from_tarball_path() {
    let temp = tempfile::tempdir().unwrap();
    let tarball = temp.path().join("archive.tgz");
    let dest = temp.path().join("dest");
    std::fs::write(&tarball, gzip_tar_file("pkg/file.txt", b"from-file")).unwrap();
    super::super::extract_tarball(&tarball, &dest).unwrap();
    assert_eq!(
        std::fs::read(dest.join("pkg/file.txt")).unwrap(),
        b"from-file"
    );
}
