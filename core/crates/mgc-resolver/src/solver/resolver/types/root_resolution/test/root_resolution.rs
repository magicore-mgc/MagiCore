//! Root request identity tests.
//! Test định danh request root.

use super::super::RootResolution;
use mgc_types::{PackageId, PackageName};

#[test]
fn keeps_request_index_spec_and_selected_package_together() {
    let request_name = PackageName::new("widget").expect("parse root name");
    let package_id = PackageId::parse("widget@1.2.0").expect("parse selected package");
    let root = RootResolution::new(
        3,
        request_name.clone(),
        "^1.0.0".to_owned(),
        package_id.clone(),
    );

    assert_eq!(root.request_index, 3);
    assert_eq!(root.request_name, request_name);
    assert_eq!(root.request_spec, "^1.0.0");
    assert_eq!(root.package_id, package_id);
}
