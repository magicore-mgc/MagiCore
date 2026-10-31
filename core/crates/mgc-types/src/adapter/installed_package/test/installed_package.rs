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
        section: Some("dependencies".to_string()),
    };
    let json = serde_json::to_string(&package).unwrap();
    let decoded: InstalledPackage = serde_json::from_str(&json).unwrap();
    assert_eq!(decoded.id, package.id);
    assert_eq!(decoded.path, package.path);
    assert_eq!(decoded.integrity, package.integrity);
    assert_eq!(decoded.is_direct, package.is_direct);
    assert_eq!(decoded.is_dev, package.is_dev);
    assert_eq!(decoded.section, package.section);
}

// An unknown section is omitted from the payload instead of emitting `null`,
// so older consumers keep decoding unchanged.
// (Section chưa biết được bỏ khỏi payload thay vì ghi `null`, để consumer cũ
// vẫn decode như cũ.)
#[test]
fn unknown_section_is_omitted_from_serialized_payload() {
    let package = InstalledPackage {
        id: PackageId::parse("sample@1.2.3").unwrap(),
        path: PathBuf::from("node_modules/sample"),
        integrity: None,
        is_direct: false,
        is_dev: false,
        section: None,
    };
    let json = serde_json::to_string(&package).unwrap();
    assert!(!json.contains("section"), "unexpected payload: {json}");
    let decoded: InstalledPackage = serde_json::from_str(&json).unwrap();
    assert_eq!(decoded.section, None);
}
