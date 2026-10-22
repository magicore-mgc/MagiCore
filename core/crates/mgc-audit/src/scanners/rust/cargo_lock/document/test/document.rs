#![allow(clippy::unwrap_used)]

use super::*;

#[test]
fn parses_a_valid_document_and_package_records() {
    let document =
        parse("version = 4\n[[package]]\nname = \"serde\"\nversion = \"1.0.0\"\n").unwrap();
    let packages = package_records(&document).unwrap().unwrap();
    assert_eq!(packages.len(), 1);
}

#[test]
fn missing_package_key_is_preserved_as_no_records() {
    let document = parse("version = 4\n").unwrap();
    assert!(package_records(&document).unwrap().is_none());
}

#[test]
fn non_array_package_key_returns_its_typed_error() {
    let document = parse("package = \"serde\"\n").unwrap();
    assert!(matches!(
        package_records(&document),
        Err(super::super::CargoLockError::PackageFieldNotArray)
    ));
}
