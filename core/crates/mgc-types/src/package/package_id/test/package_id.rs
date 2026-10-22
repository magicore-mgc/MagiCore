//! Checks package identity behavior at its owner.
//! Kiểm tra hành vi identity package tại owner gần nhất.

use super::super::PackageId;

#[test]
fn parses_scoped_package_and_preserves_version() {
    let id = PackageId::parse("@scope/pkg@1.2.3").unwrap();
    assert_eq!(id.name_str(), "@scope/pkg");
    assert_eq!(id.version().to_string(), "1.2.3");
}

#[test]
fn rejects_missing_package_separator() {
    assert!(PackageId::parse("pkg").is_err());
}
