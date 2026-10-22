//! Tests for Flutter SDK package selection and missing-SDK failures.
//! Test chọn package Flutter SDK và lỗi thiếu SDK.

use super::*;

#[test]
fn flutter_package_config_includes_only_declared_sdk_packages() {
    let project = tempfile::tempdir().unwrap();
    let sdk = tempfile::tempdir().unwrap();
    let cache = tempfile::tempdir().unwrap();
    std::fs::write(
        project.path().join("pubspec.yaml"),
        concat!(
            "name: sample\ndependencies:\n  flutter:\n    sdk: flutter\n",
            "dev_dependencies:\n  flutter_test:\n    sdk: flutter\n",
        ),
    )
    .unwrap();
    for (name, version) in [
        ("flutter", ">=3.4.0-0 <4.0.0"),
        ("flutter_test", ">=3.4.0-0 <4.0.0"),
    ] {
        let package = sdk.path().join("packages").join(name);
        std::fs::create_dir_all(package.join("lib")).unwrap();
        std::fs::write(
            package.join("pubspec.yaml"),
            format!("name: {name}\nenvironment:\n  sdk: '{version}'\n"),
        )
        .unwrap();
    }
    write_flutter_package_config_with_sdk_root(
        &ResolvedGraph::empty(),
        project.path(),
        cache.path(),
        Some(sdk.path()),
    )
    .unwrap();

    let config: serde_json::Value = serde_json::from_slice(
        &std::fs::read(project.path().join(".dart_tool/package_config.json")).unwrap(),
    )
    .unwrap();
    let packages = config["packages"].as_array().unwrap();
    let names: Vec<_> = packages
        .iter()
        .filter_map(|entry| entry["name"].as_str())
        .collect();
    assert_eq!(names, ["flutter", "flutter_test", "sample"]);
    assert!(packages.iter().any(|entry| {
        entry["name"] == "sample"
            && entry["rootUri"]
                == url::Url::from_directory_path(project.path())
                    .unwrap()
                    .as_str()
    }));
    assert!(
        packages
            .iter()
            .all(|entry| entry["rootUri"].as_str().unwrap().starts_with("file://"))
    );
}

#[test]
fn flutter_package_config_fails_closed_when_declared_sdk_package_is_missing() {
    let project = tempfile::tempdir().unwrap();
    let sdk = tempfile::tempdir().unwrap();
    let cache = tempfile::tempdir().unwrap();
    std::fs::write(
        project.path().join("pubspec.yaml"),
        "name: sample\ndependencies:\n  flutter:\n    sdk: flutter\n",
    )
    .unwrap();
    let error = write_flutter_package_config_with_sdk_root(
        &ResolvedGraph::empty(),
        project.path(),
        cache.path(),
        Some(sdk.path()),
    )
    .unwrap_err();
    assert!(error.to_string().contains("SDK package 'flutter'"));
    assert!(
        !project.path().join(".dart_tool").exists(),
        "validation error must not leave a partial package config directory"
    );
}
