//! Verifies inspection is non-persistent and checks actual write access.
//! Kiểm tra inspection không tạo path và thử quyền ghi thật.

use std::fs;

#[test]
fn missing_store_is_reported_without_creating_it() {
    let root = tempfile::tempdir().unwrap();
    let directory = root.path().join("missing").join("store");
    let snapshot = super::super::inspect_at(directory.clone());

    assert_eq!(snapshot.directory, directory);
    assert!(!snapshot.writable);
    assert!(matches!(
        snapshot.status,
        super::super::StoreStatus::Missing
    ));
    assert!(!directory.exists());
}

#[test]
fn writable_existing_directory_leaves_no_probe_artifact() {
    let root = tempfile::tempdir().unwrap();
    let snapshot = super::super::inspect_at(root.path().to_path_buf());

    assert!(snapshot.writable);
    assert!(matches!(
        snapshot.status,
        super::super::StoreStatus::Writable
    ));
    assert_eq!(fs::read_dir(root.path()).unwrap().count(), 0);
}

#[test]
fn existing_file_is_not_reported_as_a_store_directory() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("store-file");
    fs::write(&path, b"preserve").unwrap();

    let snapshot = super::super::inspect_at(path.clone());

    assert!(!snapshot.writable);
    assert!(matches!(
        snapshot.status,
        super::super::StoreStatus::NotDirectory
    ));
    assert_eq!(fs::read(path).unwrap(), b"preserve");
}

#[cfg(unix)]
#[test]
fn broken_symlink_is_not_misreported_as_a_missing_store() {
    use std::os::unix::fs::symlink;

    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("store-link");
    symlink(root.path().join("absent-target"), &path).unwrap();

    let snapshot = super::super::inspect_at(path.clone());

    assert!(!snapshot.writable);
    assert!(matches!(
        snapshot.status,
        super::super::StoreStatus::BrokenSymlink { path: ref broken_path }
            if broken_path == &path
    ));
    assert!(fs::symlink_metadata(path).unwrap().file_type().is_symlink());
}

#[cfg(unix)]
#[test]
fn broken_symlink_in_store_path_ancestor_is_reported_with_its_path() {
    use std::os::unix::fs::symlink;

    let root = tempfile::tempdir().unwrap();
    let broken_parent = root.path().join("broken-parent");
    symlink(root.path().join("absent-target"), &broken_parent).unwrap();
    let store = broken_parent.join("store");

    let snapshot = super::super::inspect_at(store);

    assert!(matches!(
        snapshot.status,
        super::super::StoreStatus::BrokenSymlink { path } if path == broken_parent
    ));
}

#[test]
fn file_in_store_path_ancestors_is_reported_as_not_a_directory() {
    let root = tempfile::tempdir().unwrap();
    let ancestor = root.path().join("not-a-directory");
    fs::write(&ancestor, b"preserve").unwrap();

    let snapshot = super::super::inspect_at(ancestor.join("store"));

    assert!(matches!(
        snapshot.status,
        super::super::StoreStatus::NotDirectory
    ));
    assert_eq!(fs::read(ancestor).unwrap(), b"preserve");
}

#[test]
fn missing_store_uses_nearest_existing_directory_for_disk_lookup() {
    let root = tempfile::tempdir().unwrap();
    let missing = root.path().join("deep").join("store");

    assert_eq!(
        super::super::nearest_existing_directory(&missing).as_deref(),
        Some(root.path())
    );
}
