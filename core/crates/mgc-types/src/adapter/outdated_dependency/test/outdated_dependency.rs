//! Outdated Dependency owner contract test.
//! Kiểm tra hợp đồng dependency lỗi thời.

use crate::adapter::OutdatedDependency;
use crate::package::PackageId;
use crate::version::Version;

#[test]
fn outdated_dependency_keeps_package_identity_and_latest_version() {
    let outdated = OutdatedDependency {
        package: PackageId::parse("sample@1.0.0").unwrap(),
        latest_version: Version::parse("2.0.0").unwrap(),
    };
    let json = serde_json::to_string(&outdated).unwrap();
    let decoded: OutdatedDependency = serde_json::from_str(&json).unwrap();
    assert_eq!(decoded.package, outdated.package);
    assert_eq!(decoded.latest_version, outdated.latest_version);
}
