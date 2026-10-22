//! Verify duplicate resolved names cannot overwrite package graph entries.
//! Kiểm tra tên package resolved trùng không thể ghi đè entry graph.

use super::super::super::FlutterPackageGraphError;
use super::*;

#[test]
fn resolved_packages_with_same_name_fail_closed() {
    let graph = support::graph(vec![
        support::package("shared", "1.0.0", &[]),
        support::package("shared", "2.0.0", &[]),
    ]);

    let error = super::resolved_package_entries(&graph, "root").unwrap_err();

    assert!(matches!(
        error,
        FlutterPackageGraphError::DuplicatePackage(name) if name == "shared"
    ));
}
