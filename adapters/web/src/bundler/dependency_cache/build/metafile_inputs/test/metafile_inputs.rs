#![allow(clippy::unwrap_used)]
//! Esbuild metafile-input provenance tests.
//! Test nguồn gốc input metafile esbuild.

use std::fs;

use tempfile::tempdir;

use super::super::{MetafileInputsError, validate_metafile_inputs};

#[test]
fn accepts_relative_input_inside_an_installed_package_root() {
    let project = tempdir().unwrap();
    let modules = project.path().join("node_modules");
    let entry = modules.join("sample/index.js");
    fs::create_dir_all(entry.parent().unwrap()).unwrap();
    fs::write(&entry, "export const value = 1;").unwrap();
    let roots = [fs::canonicalize(&modules).unwrap()];

    let result = validate_metafile_inputs(
        r#"{"inputs":{"node_modules/sample/index.js":{}}}"#,
        project.path(),
        &roots,
    );

    assert!(result.is_ok());
}

#[test]
fn accepts_absolute_input_inside_an_installed_package_root() {
    let project = tempdir().unwrap();
    let modules = project.path().join("node_modules");
    let entry = modules.join("sample/index.js");
    fs::create_dir_all(entry.parent().unwrap()).unwrap();
    fs::write(&entry, "export const value = 1;").unwrap();
    let roots = [fs::canonicalize(&modules).unwrap()];
    let mut inputs = serde_json::Map::new();
    inputs.insert(entry.to_string_lossy().into_owned(), serde_json::json!({}));
    let metadata = serde_json::json!({"inputs": inputs});

    let result = validate_metafile_inputs(&metadata.to_string(), project.path(), &roots);

    assert!(result.is_ok());
}

#[test]
fn rejects_a_parent_traversal_input_before_bundle_output_is_accepted() {
    let project = tempdir().unwrap();
    let modules = project.path().join("node_modules");
    fs::create_dir_all(modules.join("sample")).unwrap();
    fs::write(project.path().join("private.js"), "private data").unwrap();
    let working_dir = project.path().join("work");
    fs::create_dir_all(&working_dir).unwrap();
    let roots = [fs::canonicalize(&modules).unwrap()];

    let result =
        validate_metafile_inputs(r#"{"inputs":{"../private.js":{}}}"#, &working_dir, &roots);

    assert!(matches!(
        result,
        Err(MetafileInputsError::OutsideAllowedRoots)
    ));
}

#[test]
fn rejects_missing_empty_or_malformed_input_metadata() {
    let project = tempdir().unwrap();
    let roots = [fs::canonicalize(project.path()).unwrap()];

    assert!(matches!(
        validate_metafile_inputs("not-json", project.path(), &roots),
        Err(MetafileInputsError::InvalidMetadata)
    ));
    assert!(matches!(
        validate_metafile_inputs(r#"{"inputs":{}}"#, project.path(), &roots),
        Err(MetafileInputsError::InvalidMetadata)
    ));
    assert!(matches!(
        validate_metafile_inputs(r#"{"inputs":{"missing.js":{}}}"#, project.path(), &roots),
        Err(MetafileInputsError::UnresolvableInput)
    ));
}

#[cfg(unix)]
#[test]
fn rejects_a_symlink_input_that_canonicalizes_outside_allowed_roots() {
    use std::os::unix::fs::symlink;

    let project = tempdir().unwrap();
    let modules = project.path().join("node_modules");
    let package = modules.join("sample");
    fs::create_dir_all(&package).unwrap();
    let outside = project.path().join("secret.js");
    fs::write(&outside, "secret").unwrap();
    symlink(&outside, package.join("linked.js")).unwrap();
    let roots = [fs::canonicalize(&modules).unwrap()];

    let result = validate_metafile_inputs(
        r#"{"inputs":{"node_modules/sample/linked.js":{}}}"#,
        project.path(),
        &roots,
    );

    assert!(matches!(
        result,
        Err(MetafileInputsError::OutsideAllowedRoots)
    ));
}
