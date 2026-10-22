//! Tests for Web package source selection.
//! Test chọn source package Web.

use super::super::implementation::find_package_source;

#[test]
fn local_node_modules_source_is_selected_before_global_roots() {
    let project = tempfile::tempdir().expect("tempdir");
    let local = project.path().join("node_modules").join("sample-package");
    std::fs::create_dir_all(&local).expect("create local package");

    let found = find_package_source(project.path(), "sample-package").expect("local source");
    assert_eq!(found, local);
}

#[test]
fn missing_explicit_local_path_keeps_its_diagnostic() {
    let project = tempfile::tempdir().expect("tempdir");
    let error = find_package_source(project.path(), "./missing-package")
        .expect_err("missing local path must be rejected");

    assert!(error.to_string().contains("local package path not found"));
}

#[test]
fn existing_absolute_path_is_canonicalized() {
    let project = tempfile::tempdir().expect("tempdir");
    let source = project.path().join("explicit-package");
    std::fs::create_dir(&source).expect("create explicit source");

    let found = find_package_source(project.path(), source.to_str().expect("UTF-8 test path"))
        .expect("absolute source");

    assert_eq!(
        found,
        std::fs::canonicalize(source).expect("canonical source")
    );
}
