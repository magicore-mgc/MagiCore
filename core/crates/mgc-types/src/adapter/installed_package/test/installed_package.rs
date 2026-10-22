//! Installed Package owner contract test.
//! Kiểm tra hợp đồng package đã cài.

use crate::adapter::InstalledPackage;
use crate::package::PackageId;
use std::path::PathBuf;

#[test]
fn installed_package_fields_round_trip_through_serde() {
    let package = InstalledPackage {
        id: PackageId::parse("sample@1.2.3").unwrap(),
        path: PathBuf::from("node_modules/sample"),
        integrity: Some("sha512-example".to_string()),
        is_direct: true,
        is_dev: false,
    };
    let json = serde_json::to_string(&package).unwrap();
    let decoded: InstalledPackage = serde_json::from_str(&json).unwrap();
    assert_eq!(decoded.id, package.id);
    assert_eq!(decoded.path, package.path);
    assert_eq!(decoded.integrity, package.integrity);
    assert_eq!(decoded.is_direct, package.is_direct);
    assert_eq!(decoded.is_dev, package.is_dev);
}
