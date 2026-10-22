//! Verify root pubspec projection and version defaults.
//! Kiểm tra projection pubspec gốc và version mặc định.

#[test]
fn root_manifest_sorts_dependencies_and_defaults_missing_version() {
    let project = tempfile::tempdir().unwrap();
    std::fs::write(
        project.path().join("pubspec.yaml"),
        "name: sample\ndependencies:\n  zeta: ^1.0.0\n  alpha: ^2.0.0\ndev_dependencies:\n  tool: null\n",
    )
    .unwrap();

    let root = super::read_root_manifest(project.path()).unwrap();

    assert_eq!(root.name, "sample");
    assert_eq!(root.version, "0.0.0");
    assert_eq!(
        root.dependencies,
        vec!["alpha".to_string(), "zeta".to_string()]
    );
    assert_eq!(root.dev_dependencies, vec!["tool".to_string()]);
}
