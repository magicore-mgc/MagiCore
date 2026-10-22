//! Tests for Web package symlink creation.
//! Test cho thao tác tạo symlink package Web.

use super::super::implementation::{
    create_package_link, ensure_link_target_available, ensure_package_parent,
};

#[test]
fn missing_link_target_is_available() {
    let temporary = tempfile::tempdir().expect("temporary directory");
    let destination = temporary.path().join("node_modules").join("sample-package");

    ensure_link_target_available(&destination, "sample-package")
        .expect("missing link target should be available");
}

#[cfg(unix)]
#[test]
fn refuses_scoped_link_parent_symlink() {
    let temporary = tempfile::tempdir().expect("temporary directory");
    let node_modules = temporary.path().join("node_modules");
    let outside = temporary.path().join("outside");
    let scope = node_modules.join("@scope");
    let link_path = scope.join("package");
    std::fs::create_dir(&node_modules).expect("create node_modules");
    std::fs::create_dir(&outside).expect("create outside directory");
    std::os::unix::fs::symlink(&outside, &scope).expect("create scope symlink");

    let error = ensure_package_parent(&node_modules, &link_path, "@scope/package", true)
        .expect_err("must not create a link through a scope symlink");
    assert_eq!(
        error.to_string(),
        "refusing to access package link '@scope/package' through a symlink or non-directory scope"
    );
    let unlink_error = ensure_package_parent(&node_modules, &link_path, "@scope/package", false)
        .expect_err("must not unlink through a scope symlink");
    assert_eq!(unlink_error.to_string(), error.to_string());
    assert!(!outside.join("package").exists());
}

#[cfg(windows)]
#[test]
fn refuses_scoped_link_parent_symlink() {
    let temporary = tempfile::tempdir().expect("temporary directory");
    let node_modules = temporary.path().join("node_modules");
    let outside = temporary.path().join("outside");
    let scope = node_modules.join("@scope");
    let link_path = scope.join("package");
    std::fs::create_dir(&node_modules).expect("create node_modules");
    std::fs::create_dir(&outside).expect("create outside directory");
    std::os::windows::fs::symlink_dir(&outside, &scope).expect("create scope symlink");

    let error = ensure_package_parent(&node_modules, &link_path, "@scope/package", true)
        .expect_err("must not create a link through a scope symlink");
    assert_eq!(
        error.to_string(),
        "refusing to access package link '@scope/package' through a symlink or non-directory scope"
    );
    let unlink_error = ensure_package_parent(&node_modules, &link_path, "@scope/package", false)
        .expect_err("must not unlink through a scope symlink");
    assert_eq!(unlink_error.to_string(), error.to_string());
    assert!(!outside.join("package").exists());
}

#[cfg(unix)]
#[test]
fn created_link_resolves_to_the_selected_source() {
    let temporary = tempfile::tempdir().expect("temporary directory");
    let source = temporary.path().join("source-package");
    let destination = temporary.path().join("node_modules").join("sample-package");
    std::fs::create_dir(&source).expect("create source package");
    std::fs::create_dir(destination.parent().expect("link parent")).expect("create link parent");

    create_package_link(&source, &destination).expect("create package symlink");

    let metadata = std::fs::symlink_metadata(&destination).expect("link metadata");
    assert!(metadata.file_type().is_symlink());
    assert_eq!(
        std::fs::canonicalize(&destination).expect("resolve link"),
        std::fs::canonicalize(&source).expect("canonical source")
    );
}

#[cfg(unix)]
#[test]
fn dangling_symlink_occupies_its_link_target() {
    let temporary = tempfile::tempdir().expect("temporary directory");
    let destination = temporary.path().join("node_modules").join("sample-package");
    std::fs::create_dir(destination.parent().expect("link parent")).expect("create link parent");
    std::os::unix::fs::symlink(temporary.path().join("missing-target"), &destination)
        .expect("create dangling symlink");

    let error = ensure_link_target_available(&destination, "sample-package")
        .expect_err("dangling symlink must occupy the path");

    assert_eq!(
        error.to_string(),
        "sample-package already exists in node_modules"
    );
}

#[cfg(windows)]
#[test]
fn dangling_directory_symlink_occupies_its_link_target() {
    let temporary = tempfile::tempdir().expect("temporary directory");
    let destination = temporary.path().join("node_modules").join("sample-package");
    std::fs::create_dir(destination.parent().expect("link parent")).expect("create link parent");
    std::os::windows::fs::symlink_dir(temporary.path().join("missing-target"), &destination)
        .expect("create dangling directory symlink");

    let error = ensure_link_target_available(&destination, "sample-package")
        .expect_err("dangling symlink must occupy the path");

    assert_eq!(
        error.to_string(),
        "sample-package already exists in node_modules"
    );
}

#[cfg(windows)]
#[test]
fn created_directory_link_resolves_to_the_selected_source() {
    let temporary = tempfile::tempdir().expect("temporary directory");
    let source = temporary.path().join("source-package");
    let destination = temporary.path().join("node_modules").join("sample-package");
    std::fs::create_dir(&source).expect("create source package");
    std::fs::create_dir(destination.parent().expect("link parent")).expect("create link parent");

    create_package_link(&source, &destination).expect("create package symlink");

    let metadata = std::fs::symlink_metadata(&destination).expect("link metadata");
    assert!(metadata.file_type().is_symlink());
    assert_eq!(
        std::fs::canonicalize(&destination).expect("resolve link"),
        std::fs::canonicalize(&source).expect("canonical source")
    );
}
