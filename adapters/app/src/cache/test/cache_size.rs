//! Cache traversal regressions for owned directories.
//! Kiểm chứng tính dung lượng không theo symlink ra ngoài cache.

use super::dir_size;

#[test]
fn owned_regular_files_are_counted_recursively() {
    let root = tempfile::tempdir().expect("temporary cache");
    std::fs::create_dir(root.path().join("nested")).expect("nested directory");
    std::fs::write(root.path().join("first"), b"abc").expect("first file");
    std::fs::write(root.path().join("nested/second"), b"12345").expect("second file");
    assert_eq!(dir_size(root.path()).expect("cache size"), 8);
}

#[cfg(unix)]
#[test]
fn linked_root_is_rejected_and_linked_entries_are_not_followed() {
    use std::os::unix::fs::symlink;
    let root = tempfile::tempdir().expect("temporary cache");
    let external = tempfile::tempdir().expect("external directory");
    std::fs::write(external.path().join("external"), b"outside").expect("external file");
    std::fs::write(root.path().join("owned"), b"abc").expect("owned file");
    symlink(external.path(), root.path().join("outside")).expect("directory link");
    symlink(root.path(), root.path().join("cycle")).expect("cyclic link");
    symlink(external.path().join("external"), root.path().join("file")).expect("file link");
    assert_eq!(dir_size(root.path()).expect("safe traversal"), 3);
    assert!(dir_size(&root.path().join("outside")).is_err());
    assert_eq!(
        std::fs::read(external.path().join("external")).expect("external unchanged"),
        b"outside"
    );
}
