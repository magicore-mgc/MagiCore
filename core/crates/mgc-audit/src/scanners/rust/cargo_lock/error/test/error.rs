#![allow(clippy::unwrap_used)]

use super::*;
use std::error::Error;

#[test]
fn parser_errors_keep_the_legacy_display_text() {
    assert_eq!(
        CargoLockError::PackageFieldNotArray.to_string(),
        "Cargo.lock package field is not an array"
    );
    assert_eq!(
        CargoLockError::PackageNameInvalid.to_string(),
        "Cargo.lock package has no valid name"
    );
    assert_eq!(
        CargoLockError::PackageVersionInvalid {
            package_name: "serde".to_string(),
        }
        .to_string(),
        "Cargo.lock package 'serde' has no valid version"
    );
}

#[test]
fn malformed_toml_keeps_its_source_error() {
    let source = toml::from_str::<toml::Value>("package = [").unwrap_err();
    let error = CargoLockError::InvalidToml(source);
    assert!(Error::source(&error).is_some());
    assert!(error.to_string().starts_with("invalid Cargo.lock TOML: "));
}
