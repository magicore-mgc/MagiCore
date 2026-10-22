//! Preserve the existing flutter_sdk_dependency_overrides_fail_closed Flutter SDK behavior assertion.
//! Giữ nguyên assertion hành vi Flutter SDK flutter_sdk_dependency_overrides_fail_closed.

#[test]
fn flutter_sdk_dependency_overrides_fail_closed() {
    let project = tempfile::tempdir().unwrap();
    let sdk = tempfile::tempdir().unwrap();
    std::fs::write(
        project.path().join("pubspec.yaml"),
        "name: sample\ndependencies:\n  flutter:\n    sdk: flutter\n",
    )
    .unwrap();
    std::fs::create_dir_all(sdk.path().join("packages/flutter/lib")).unwrap();
    std::fs::write(
        sdk.path().join("packages/flutter/pubspec.yaml"),
        "name: flutter\ndependency_overrides:\n  meta: 1.0.0\n",
    )
    .unwrap();

    let base =
        crate::manifest::parse_manifest(crate::AppLanguage::Flutter, project.path()).unwrap();
    let error = crate::manifest::flutter::expand_flutter_sdk_dependencies_with_root(
        project.path(),
        &base,
        sdk.path(),
    )
    .expect_err("SDK overrides must not silently be ignored");
    assert!(error.to_string().contains("override/workspace semantics"));
}
