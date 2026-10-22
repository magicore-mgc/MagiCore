//! Preserve the existing flutter_sdk_package_dependencies_enter_native_pub_graph Flutter SDK behavior assertion.
//! Giữ nguyên assertion hành vi Flutter SDK flutter_sdk_package_dependencies_enter_native_pub_graph.

use mgc_types::Version;

#[test]
fn flutter_sdk_package_dependencies_enter_native_pub_graph() {
    let project = tempfile::tempdir().unwrap();
    let sdk = tempfile::tempdir().unwrap();
    std::fs::write(
        project.path().join("pubspec.yaml"),
        "name: sample\ndependencies:\n  flutter:\n    sdk: flutter\n  meta: ^1.17.0\ndev_dependencies:\n  flutter_test:\n    sdk: flutter\n",
    )
    .unwrap();
    std::fs::create_dir_all(sdk.path().join("packages/flutter/lib")).unwrap();
    std::fs::write(
        sdk.path().join("packages/flutter/pubspec.yaml"),
        "name: flutter\ndependencies:\n  collection: ^1.18.0\n  meta: ^1.18.0\n",
    )
    .unwrap();
    std::fs::create_dir_all(sdk.path().join("packages/flutter_test/lib")).unwrap();
    std::fs::write(
        sdk.path().join("packages/flutter_test/pubspec.yaml"),
        "name: flutter_test\ndependencies:\n  flutter:\n    sdk: flutter\n  sky_engine:\n    sdk: flutter\n  matcher: ^0.12.16\n",
    )
    .unwrap();
    std::fs::create_dir_all(sdk.path().join("bin/cache/pkg/sky_engine/lib")).unwrap();
    std::fs::write(
        sdk.path().join("bin/cache/pkg/sky_engine/pubspec.yaml"),
        "name: sky_engine\n",
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
    assert!(expanded.find_dep("collection").is_some());
    assert!(expanded.find_dep("matcher").is_some());
    let meta_range = &expanded.find_dep("meta").unwrap().range;
    assert!(meta_range.matches(&Version::parse("1.18.0").unwrap()));
    assert!(!meta_range.matches(&Version::parse("1.17.0").unwrap()));
    assert!(
        expanded
            .dependencies
            .iter()
            .any(|dependency| dependency.name.as_str() == "collection")
    );
    assert!(
        expanded
            .dev_dependencies
            .iter()
            .any(|dependency| dependency.name.as_str() == "matcher")
    );
    assert!(expanded.find_dep("sky_engine").is_none());

    let names =
        crate::manifest::flutter::flutter_sdk_package_names(project.path(), sdk.path()).unwrap();
    assert_eq!(names, ["flutter", "flutter_test", "sky_engine"]);
}
