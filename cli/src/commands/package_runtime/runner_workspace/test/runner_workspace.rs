//! Focused runner workspace regression tests.
//! Test hồi quy tập trung cho runner workspace.

use super::*;

#[test]
fn creates_registry_project_with_owned_request_and_copies_explicit_npmrc() {
    let caller = tempfile::tempdir().expect("isolated caller");
    let data = tempfile::tempdir().expect("isolated data root");
    std::fs::write(
        caller.path().join(".npmrc"),
        "registry=https://registry.example/\n",
    )
    .expect("write explicit registry configuration");

    let workspace = prepare_runner_workspace_at(
        data.path(),
        caller.path(),
        "@scope/initializer",
        "^2.3.0",
        PackageRunPurpose::CreateInitializer,
    )
    .expect("prepare MGC-managed runner project");

    let package: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(workspace.project_root.join("package.json"))
            .expect("read generated runner manifest"),
    )
    .expect("parse generated runner manifest");
    assert_eq!(
        package["dependencies"]["@scope/initializer"],
        serde_json::Value::String("^2.3.0".to_string())
    );
    assert!(
        workspace
            .project_root
            .starts_with(data.path().canonicalize().expect("canonical data root"))
    );
    assert_eq!(
        std::fs::read_to_string(workspace.project_root.join(".npmrc")).expect("read copied npmrc"),
        "registry=https://registry.example/\n"
    );
}
