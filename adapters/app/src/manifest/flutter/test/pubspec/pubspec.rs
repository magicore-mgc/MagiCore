//! Test Flutter SDK entry preservation during pubspec rewrite.
//! Test giữ entry Flutter SDK khi ghi lại pubspec.

#[test]
fn flutter_pubspec_writer_preserves_all_sdk_dependencies() {
    let project = tempfile::tempdir().unwrap();
    std::fs::write(
        project.path().join("pubspec.yaml"),
        "name: sample\ndependencies:\n  flutter:\n    sdk: flutter\ndev_dependencies:\n  flutter_test:\n    sdk: flutter\n  integration_test:\n    sdk: flutter\n",
    )
    .unwrap();
    let manifest =
        crate::manifest::parse_manifest(crate::AppLanguage::Flutter, project.path()).unwrap();

    crate::manifest::flutter::write_pubspec(project.path(), &manifest).unwrap();

    let rewritten: serde_yaml::Value = serde_yaml::from_str(
        &std::fs::read_to_string(project.path().join("pubspec.yaml")).unwrap(),
    )
    .unwrap();
    assert_eq!(
        rewritten["dependencies"]["flutter"]["sdk"].as_str(),
        Some("flutter")
    );
    for name in ["flutter_test", "integration_test"] {
        assert_eq!(
            rewritten["dev_dependencies"][name]["sdk"].as_str(),
            Some("flutter"),
            "writer dropped SDK test dependency {name}"
        );
    }
}
