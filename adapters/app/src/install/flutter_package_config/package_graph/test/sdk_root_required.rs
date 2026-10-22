//! Verify selected SDK packages require a known SDK root.
//! Kiểm tra package SDK được chọn phải có SDK root.

use super::super::FlutterPackageGraphError;
use super::*;

#[test]
fn selected_sdk_packages_require_sdk_root_before_projection() {
    let project = tempfile::tempdir().unwrap();
    std::fs::write(project.path().join("pubspec.yaml"), "name: sample\n").unwrap();

    let error = super::build_flutter_package_graph_inner(
        &ResolvedGraph::empty(),
        project.path(),
        None,
        &["flutter".to_string()],
    )
    .unwrap_err();

    assert!(matches!(error, FlutterPackageGraphError::MissingSdkRoot));
}
