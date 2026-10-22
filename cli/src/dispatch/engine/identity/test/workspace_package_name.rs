use super::*;

#[test]
fn package_name_reader_returns_manifest_name_when_present() {
    let workspace = tempfile::tempdir().unwrap();
    std::fs::write(
        workspace.path().join("package.json"),
        r#"{"name":"@fixture/web-app"}"#,
    )
    .unwrap();

    assert_eq!(
        workspace_package_name(workspace.path()).unwrap().as_deref(),
        Some("@fixture/web-app")
    );
}

#[test]
fn package_name_reader_supports_cargo_and_python_workspace_manifests() {
    let rust_workspace = tempfile::tempdir().unwrap();
    std::fs::write(
        rust_workspace.path().join("Cargo.toml"),
        "[package]\nname = \"rust-fixture\"\nversion = \"0.1.0\"\n",
    )
    .unwrap();
    assert_eq!(
        workspace_package_name(rust_workspace.path())
            .unwrap()
            .as_deref(),
        Some("rust-fixture")
    );

    let python_workspace = tempfile::tempdir().unwrap();
    std::fs::write(
        python_workspace.path().join("pyproject.toml"),
        "[project]\nname = \"python-fixture\"\n",
    )
    .unwrap();
    assert_eq!(
        workspace_package_name(python_workspace.path())
            .unwrap()
            .as_deref(),
        Some("python-fixture")
    );
}

#[test]
fn package_name_reader_returns_none_when_manifest_is_missing() {
    let workspace = tempfile::tempdir().unwrap();

    assert_eq!(workspace_package_name(workspace.path()).unwrap(), None);
}

#[test]
fn package_name_reader_propagates_invalid_manifest_errors() {
    let workspace = tempfile::tempdir().unwrap();
    std::fs::write(workspace.path().join("package.json"), "{").unwrap();

    assert!(workspace_package_name(workspace.path()).is_err());
}
