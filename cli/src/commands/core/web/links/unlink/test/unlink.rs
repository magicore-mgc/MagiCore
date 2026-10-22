//! Tests for Web package symlink removal.
//! Test cho thao tác gỡ symlink package Web.

use super::super::implementation::remove_package_entry;

#[test]
fn ordinary_package_directory_is_rejected_without_deleting_contents() {
    let temporary = tempfile::tempdir().expect("temporary directory");
    let package = temporary.path().join("node_modules").join("sample-package");
    std::fs::create_dir_all(&package).expect("create package directory");
    let marker = package.join("keep.txt");
    std::fs::write(&marker, "preserve package data").expect("write marker");

    let error = remove_package_entry(&package, "sample-package")
        .expect_err("ordinary directory must not be removed");

    assert_eq!(
        error.to_string(),
        "refusing to remove non-link package entry 'sample-package' from node_modules"
    );
    assert!(marker.is_file());
}

#[test]
fn ordinary_package_file_is_rejected_without_deleting_contents() {
    let temporary = tempfile::tempdir().expect("temporary directory");
    let package = temporary.path().join("node_modules").join("sample-package");
    std::fs::create_dir_all(package.parent().expect("node_modules directory"))
        .expect("create node_modules");
    std::fs::write(&package, "preserve package data").expect("write package file");

    let error = remove_package_entry(&package, "sample-package")
        .expect_err("ordinary file must not be removed");

    assert_eq!(
        error.to_string(),
        "refusing to remove non-link package entry 'sample-package' from node_modules"
    );
    assert!(package.is_file());
}

#[cfg(unix)]
#[test]
fn removing_directory_link_does_not_remove_its_target() {
    let temporary = tempfile::tempdir().expect("temporary directory");
    let target = temporary.path().join("target-package");
    let link = temporary.path().join("node_modules").join("sample-package");
    std::fs::create_dir(&target).expect("create target package");
    std::fs::create_dir(link.parent().expect("link parent")).expect("create link parent");
    std::os::unix::fs::symlink(&target, &link).expect("create package symlink");

    remove_package_entry(&link, "sample-package").expect("remove package link");

    assert!(target.is_dir());
    assert!(std::fs::symlink_metadata(&link).is_err());
}

#[cfg(unix)]
#[test]
fn removes_dangling_symlink_without_touching_missing_target() {
    let temporary = tempfile::tempdir().expect("temporary directory");
    let missing_target = temporary.path().join("missing-package");
    let link = temporary.path().join("node_modules").join("sample-package");
    std::fs::create_dir(link.parent().expect("link parent")).expect("create link parent");
    std::os::unix::fs::symlink(&missing_target, &link).expect("create dangling symlink");

    remove_package_entry(&link, "sample-package").expect("remove dangling package link");

    assert!(!missing_target.exists());
    assert!(std::fs::symlink_metadata(&link).is_err());
}

#[cfg(windows)]
#[test]
fn removes_dangling_directory_symlink_without_touching_missing_target() {
    let temporary = tempfile::tempdir().expect("temporary directory");
    let missing_target = temporary.path().join("missing-package");
    let link = temporary.path().join("node_modules").join("sample-package");
    std::fs::create_dir(link.parent().expect("link parent")).expect("create link parent");
    std::os::windows::fs::symlink_dir(&missing_target, &link)
        .expect("create dangling directory symlink");

    remove_package_entry(&link, "sample-package").expect("remove dangling package link");

    assert!(!missing_target.exists());
    assert!(std::fs::symlink_metadata(&link).is_err());
}

#[cfg(windows)]
#[test]
fn removing_directory_link_does_not_remove_its_target() {
    let temporary = tempfile::tempdir().expect("temporary directory");
    let target = temporary.path().join("target-package");
    let link = temporary.path().join("node_modules").join("sample-package");
    std::fs::create_dir(&target).expect("create target package");
    std::fs::create_dir(link.parent().expect("link parent")).expect("create link parent");
    std::os::windows::fs::symlink_dir(&target, &link).expect("create package symlink");

    remove_package_entry(&link, "sample-package").expect("remove package link");

    assert!(target.is_dir());
    assert!(std::fs::symlink_metadata(&link).is_err());
}
