//! Existing dependency-gate test — test dependency gate hiện có.
//! This subject keeps its complete pre-refactor assertion block.
//! Giữ nguyên toàn bộ assertion trước refactor.

#[test]
fn cloud_gate_requires_embedded_js_manifest_and_rejects_compat() {
    use crate::commands::dep_gate::{DepOp, gate_cloud_project};

    let missing_manifest = tempfile::tempdir().unwrap();
    assert!(gate_cloud_project(missing_manifest.path(), "cdk", DepOp::Install, None).is_err());
    assert!(
        gate_cloud_project(
            missing_manifest.path(),
            "cdk",
            DepOp::OfflineReinstall,
            None
        )
        .is_err()
    );

    let js_project = tempfile::tempdir().unwrap();
    std::fs::write(js_project.path().join("package.json"), "{}\n").unwrap();
    assert!(gate_cloud_project(js_project.path(), "cdk", DepOp::Install, None).is_ok());
    assert!(gate_cloud_project(js_project.path(), "pulumi", DepOp::Add, None).is_ok());
    assert!(gate_cloud_project(js_project.path(), "cdk", DepOp::OfflineReinstall, None).is_ok());
    assert!(gate_cloud_project(js_project.path(), "terraform", DepOp::Install, None).is_err());
    assert!(gate_cloud_project(js_project.path(), "cdk", DepOp::Install, Some("npm")).is_err());
}
