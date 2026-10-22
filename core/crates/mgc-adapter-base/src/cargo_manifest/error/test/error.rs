//! Cargo manifest errors preserve stable text and TOML causes.
//! Lỗi Cargo manifest giữ text ổn định và TOML cause gốc.

use super::{
    CargoManifestError, CargoManifestErrorKind, CargoManifestRecovery, invalid_manifest,
    parse_error, serialization_error,
};
use mgc_types::MgError;
use std::error::Error;

#[test]
fn parse_error_keeps_display_kind_recovery_and_source() {
    let source = toml::from_str::<toml::Value>("invalid = [").unwrap_err();
    let MgError::Anyhow(error) = parse_error(source) else {
        panic!("Cargo TOML parse error lost its typed owner");
    };
    let typed = error.downcast_ref::<CargoManifestError>().unwrap();

    assert!(typed.message().starts_with("parse Cargo.toml: "));
    assert_eq!(typed.kind(), CargoManifestErrorKind::TomlParse);
    assert_eq!(typed.recovery(), CargoManifestRecovery::CorrectManifest);
    assert!(Error::source(typed).is_some());
}

#[test]
fn invalid_shape_keeps_message_without_fabricated_source() {
    let MgError::Anyhow(error) = invalid_manifest("Cargo.toml not a table") else {
        panic!("Cargo shape error lost its typed owner");
    };
    let typed = error.downcast_ref::<CargoManifestError>().unwrap();

    assert_eq!(typed.to_string(), "Cargo.toml not a table");
    assert_eq!(typed.kind(), CargoManifestErrorKind::ManifestStructure);
    assert_eq!(typed.recovery(), CargoManifestRecovery::CorrectManifest);
    assert!(Error::source(typed).is_none());
}

#[test]
fn serialization_error_keeps_source_and_recovery_classification() {
    struct FailingSerialization;

    impl serde::Serialize for FailingSerialization {
        fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            Err(<S::Error as serde::ser::Error>::custom(
                "injected serialization failure",
            ))
        }
    }

    let source = toml::to_string(&FailingSerialization).unwrap_err();
    let MgError::Anyhow(error) = serialization_error(source) else {
        panic!("Cargo serialization error lost its typed owner");
    };
    let typed = error.downcast_ref::<CargoManifestError>().unwrap();

    assert_eq!(typed.message(), "injected serialization failure");
    assert_eq!(typed.kind(), CargoManifestErrorKind::Serialization);
    assert_eq!(
        typed.recovery(),
        CargoManifestRecovery::InspectSerialization
    );
    assert!(Error::source(typed).is_some());
}
