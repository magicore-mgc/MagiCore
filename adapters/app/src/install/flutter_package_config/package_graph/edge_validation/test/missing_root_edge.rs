//! Verify unresolved root dependencies are rejected.
//! Kiểm tra dependency root chưa resolve bị từ chối.

use super::*;

#[test]
fn root_dependency_to_missing_entry_fails_closed() {
    let error =
        super::validate_package_edges(&entries(&[]), &["missing".to_string()], &[]).unwrap_err();

    assert!(matches!(
        error,
        FlutterPackageGraphError::UnresolvedRootDependency(name) if name == "missing"
    ));
}
