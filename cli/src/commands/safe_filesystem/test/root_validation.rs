use super::super::{SafeFilesystemError, canonical_directory, existing_directory_under};
#[cfg(any(unix, windows))]
use super::symlink_support::symlink_directory;

#[test]
fn canonical_directory_rejects_non_directory_and_missing_path() {
    let temp = tempfile::tempdir().unwrap();
    let file = temp.path().join("file");
    std::fs::write(&file, b"file").unwrap();

    assert!(matches!(
        canonical_directory(&file),
        Err(SafeFilesystemError::NotDirectory { .. })
    ));
    assert!(matches!(
        canonical_directory(&temp.path().join("missing")),
        Err(SafeFilesystemError::InspectDirectory { .. })
    ));
}

#[cfg(any(unix, windows))]
#[test]
fn canonical_directory_rejects_symlink() {
    let temp = tempfile::tempdir().unwrap();
    let target = temp.path().join("target");
    std::fs::create_dir(&target).unwrap();
    let link = temp.path().join("link");
    symlink_directory(&target, &link).unwrap();

    assert!(matches!(
        canonical_directory(&link),
        Err(SafeFilesystemError::SymlinkedDirectory { .. })
    ));
}

#[test]
fn existing_directory_under_returns_none_for_missing_and_rejects_escape() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().join("root");
    let outside = temp.path().join("outside");
    std::fs::create_dir(&root).unwrap();
    std::fs::create_dir(&outside).unwrap();

    assert_eq!(
        existing_directory_under(&root.join("missing"), &root).unwrap(),
        None
    );
    assert!(matches!(
        existing_directory_under(&outside, &root),
        Err(SafeFilesystemError::DirectoryEscapesParent { .. })
    ));
}
