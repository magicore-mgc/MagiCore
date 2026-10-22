//! Verify an unresolved package dependency is rejected.
//! Kiểm tra dependency package chưa resolve bị từ chối.

use super::*;

#[test]
fn package_edge_to_missing_entry_fails_closed() {
    let graph = entries(&[("consumer", &["missing"])]);
    let error = super::validate_package_edges(&graph, &[], &[]).unwrap_err();

    assert!(matches!(
        error,
        FlutterPackageGraphError::UnresolvedPackageEdge { package, dependency }
            if package == "consumer" && dependency == "missing"
    ));
}
