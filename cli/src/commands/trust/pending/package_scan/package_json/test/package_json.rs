//! Package-manifest tests — test kiểm tra manifest package.

#[test]
fn malformed_installed_package_manifest_is_not_silently_skipped() {
    let temp = tempfile::tempdir().expect("temporary directory must be created");
    let node_modules = temp.path().join("node_modules");
    let package = node_modules.join("sample-package");
    std::fs::create_dir_all(&package).expect("package directory must be created");
    std::fs::write(package.join("package.json"), "{not-json")
        .expect("malformed package manifest must be written");

    let mut scripted = std::collections::HashSet::new();
    let result = super::check_package_json(&package, "sample-package", &mut scripted);

    assert!(
        result.is_err(),
        "a malformed installed package manifest must not disappear from trust review"
    );
}

#[test]
fn package_json_with_non_object_scripts_is_not_treated_as_hook_free() {
    let temp = tempfile::tempdir().expect("temporary directory must be created");
    let node_modules = temp.path().join("node_modules");
    let package = node_modules.join("sample-package");
    std::fs::create_dir_all(&package).expect("package directory must be created");
    std::fs::write(
        package.join("package.json"),
        r#"{"name":"sample-package","version":"1.0.0","scripts":["postinstall"]}"#,
    )
    .expect("malformed scripts shape must be written");

    let mut scripted = std::collections::HashSet::new();
    let result = super::check_package_json(&package, "sample-package", &mut scripted);

    assert!(
        result.is_err(),
        "a non-object scripts field must not be reported as hook-free"
    );
}

#[test]
fn non_object_package_json_root_is_not_treated_as_hook_free() {
    let temp = tempfile::tempdir().expect("temporary directory must be created");
    let node_modules = temp.path().join("node_modules");
    let package = node_modules.join("sample-package");
    std::fs::create_dir_all(&package).expect("package directory must be created");
    std::fs::write(package.join("package.json"), "[]")
        .expect("non-object manifest root must be written");

    let mut scripted = std::collections::HashSet::new();
    let result = super::check_package_json(&package, "sample-package", &mut scripted);

    assert!(
        result.is_err(),
        "a non-object package manifest must not be reported as hook-free"
    );
}
