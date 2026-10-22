//! Focused tests for Cargo manifest error mapping.
//! Test tập trung cho ánh xạ lỗi manifest Cargo.

use super::CargoManifestError;
use mgc_types::MgError;

#[test]
fn toml_parse_mapping_keeps_the_parse_source_chain() {
    let source = toml::from_str::<toml::Value>("[package\n").unwrap_err();
    let local = CargoManifestError::TomlParse(source);
    assert!(std::error::Error::source(&local).is_some());

    let shared = MgError::from(local);
    match shared {
        MgError::Anyhow(error) => assert!(error.chain().count() >= 2),
        other => panic!("expected retained source chain, got {other}"),
    }
}

#[test]
fn project_file_error_is_passed_through_without_reclassification() {
    let shared = MgError::from(CargoManifestError::ProjectFile(MgError::Other(
        "Cargo.toml is not a regular file".to_string(),
    )));
    match shared {
        MgError::Other(message) => assert_eq!(message, "Cargo.toml is not a regular file"),
        other => panic!("expected the original project-file error, got {other}"),
    }
}

#[test]
fn validation_errors_keep_their_existing_rendered_text() {
    let shared = MgError::from(CargoManifestError::DirectRootsMismatch);
    match shared {
        MgError::Other(message) => assert_eq!(
            message,
            "mgc.lock direct roots do not match Cargo.toml [dependencies]; run `mgc install`"
        ),
        other => panic!("expected validation error, got {other}"),
    }
}
