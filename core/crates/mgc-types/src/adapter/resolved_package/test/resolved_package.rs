//! Resolved Package owner contract test.
//! Kiểm tra hợp đồng package đã resolve.

use crate::adapter::ResolvedPackage;
use crate::package::PackageId;

#[test]
fn legacy_resolved_package_without_peer_dependencies_defaults_empty() {
    let package = ResolvedPackage {
        id: PackageId::parse("sample@1.0.0").unwrap(),
        integrity: "sha512-example".to_string(),
        tarball_url: "https://registry.invalid/sample.tgz".to_string(),
        deps: vec![],
        peer_deps: vec![],
        direct: true,
        dev: false,
    };
    let mut value = serde_json::to_value(package).unwrap();
    value.as_object_mut().unwrap().remove("peer_deps");
    let decoded: ResolvedPackage = serde_json::from_value(value).unwrap();
    assert!(decoded.peer_deps.is_empty());
}
