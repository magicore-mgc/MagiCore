// Exact duplicate identity behavior test.
// Test hành vi identity khi bản ghi trùng hoàn toàn.
use super::super::index_pkgs;
use crate::Package;

#[test]
fn identical_records_with_one_identity_collapse_to_one_entry() {
    let package = Package {
        name: "shared".to_string(),
        version: "1.0.0".to_string(),
        ..Default::default()
    };

    let indexed = index_pkgs(&[package.clone(), package]).expect("identical records merge");

    assert_eq!(indexed.len(), 1);
}

#[test]
fn conflicting_records_with_one_identity_fail_closed() {
    let first = Package {
        name: "shared".to_string(),
        version: "1.0.0".to_string(),
        ..Default::default()
    };
    let second = Package {
        name: "shared".to_string(),
        version: "2.0.0".to_string(),
        ..Default::default()
    };

    let error = index_pkgs(&[first, second]).expect_err("one identity cannot keep two records");

    assert_eq!(error.name, "shared");
    assert_eq!(error.base_version, None);
    assert_eq!(error.ours_version, "1.0.0");
    assert_eq!(error.theirs_version, "2.0.0");
}
