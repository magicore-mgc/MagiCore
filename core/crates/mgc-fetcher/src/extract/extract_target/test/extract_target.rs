//! Covers safe parent creation and symlink refusal.
//! Kiểm tra tạo parent an toàn và từ chối symlink.

use std::path::Path;

#[test]
fn creates_missing_parent_components_below_root() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().join("root");
    std::fs::create_dir(&root).unwrap();
    let target = super::super::ensure_extract_file_target(&root, Path::new("a/b/file")).unwrap();
    assert_eq!(target, root.join("a/b/file"));
    assert!(root.join("a/b").is_dir());
}

#[cfg(unix)]
#[test]
fn refuses_preexisting_symlink_parent() {
    use std::os::unix::fs::symlink;

    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().join("root");
    let outside = temp.path().join("outside");
    std::fs::create_dir(&root).unwrap();
    std::fs::create_dir(&outside).unwrap();
    symlink(&outside, root.join("linked")).unwrap();
    let error =
        super::super::ensure_extract_file_target(&root, Path::new("linked/file")).unwrap_err();
    assert!(matches!(
        error,
        super::super::super::ExtractionError::ParentSymlink(_)
    ));
    assert!(!outside.join("file").exists());
}

#[cfg(unix)]
#[test]
fn refuses_symlink_extraction_root() {
    use std::os::unix::fs::symlink;

    let temp = tempfile::tempdir().unwrap();
    let real = temp.path().join("real");
    let linked = temp.path().join("linked");
    std::fs::create_dir(&real).unwrap();
    symlink(&real, &linked).unwrap();
    assert!(matches!(
        super::super::ensure_extract_root(&linked),
        Err(super::super::super::ExtractionError::RootSymlink(_))
    ));
}
