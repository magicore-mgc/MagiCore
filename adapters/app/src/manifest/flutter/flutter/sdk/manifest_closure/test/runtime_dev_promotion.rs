//! Preserve the existing flutter_sdk_runtime_dependency_promotes_project_optional_package_to_runtime Flutter SDK behavior assertion.
//! Giữ nguyên assertion hành vi Flutter SDK flutter_sdk_runtime_dependency_promotes_project_optional_package_to_runtime.

use mgc_types::Version;

#[test]
fn flutter_sdk_runtime_dependency_promotes_project_optional_package_to_runtime() {
    let project = tempfile::tempdir().unwrap();
    let sdk = tempfile::tempdir().unwrap();
    std::fs::write(
        project.path().join("pubspec.yaml"),
        "name: sample\ndependencies:\n  flutter:\n    sdk: flutter\ndev_dependencies:\n  meta: ^1.17.0\n",
    )
    .unwrap();
    std::fs::create_dir_all(sdk.path().join("packages/flutter/lib")).unwrap();
    std::fs::write(
        sdk.path().join("packages/flutter/pubspec.yaml"),
        "name: flutter\ndependencies:\n  meta: ^1.18.0\n",
    )
    .unwrap();

    let base =
        crate::manifest::parse_manifest(crate::AppLanguage::Flutter, project.path()).unwrap();
    let expanded = crate::manifest::flutter::expand_flutter_sdk_dependencies_with_root(
        project.path(),
        &base,
        sdk.path(),
    )
    .unwrap();

    assert!(
        expanded
            .dependencies
            .iter()
            .any(|dependency| dependency.name.as_str() == "meta"),
        "a Flutter SDK runtime dependency must promote the same project dev dependency"
    );
    assert!(
        !expanded
            .dev_dependencies
            .iter()
            .any(|dependency| dependency.name.as_str() == "meta"),
        "meta must not remain dev-only when Flutter requires it at runtime"
    );
    assert!(
        expanded
            .find_dep("meta")
            .unwrap()
            .range
            .matches(&Version::parse("1.18.0").unwrap())
    );
}
