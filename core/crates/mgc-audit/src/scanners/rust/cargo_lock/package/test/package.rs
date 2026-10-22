#![allow(clippy::unwrap_used)]

use super::*;
use toml::Value;

#[test]
fn returns_name_and_version_from_a_package_record() {
    let package: Value = toml::from_str("name = \"serde\"\nversion = \"1.0.0\"\n").unwrap();
    assert_eq!(name_version(&package).unwrap(), ("serde", "1.0.0"));
}

#[test]
fn missing_name_and_version_have_distinct_typed_errors() {
    let missing_name: Value = toml::from_str("version = \"1.0.0\"\n").unwrap();
    assert!(matches!(
        name_version(&missing_name),
        Err(super::super::CargoLockError::PackageNameInvalid)
    ));

    let missing_version: Value = toml::from_str("name = \"serde\"\n").unwrap();
    assert!(matches!(
        name_version(&missing_version),
        Err(super::super::CargoLockError::PackageVersionInvalid { package_name })
            if package_name == "serde"
    ));
}

#[test]
fn source_is_present_only_when_it_is_a_string() {
    let registry: Value =
        toml::from_str("source = \"registry+https://index.crates.io/\"\n").unwrap();
    let non_string: Value = toml::from_str("source = 42\n").unwrap();
    let absent: Value = toml::from_str("name = \"local\"\n").unwrap();
    assert_eq!(source(&registry), Some("registry+https://index.crates.io/"));
    assert_eq!(source(&non_string), None);
    assert_eq!(source(&absent), None);
}
