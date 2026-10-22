#![allow(clippy::unwrap_used)]
//! Generator error mapping tests — Kiểm thử ánh xạ lỗi generator

use super::super::*;

#[test]
fn invalid_lockfile_keeps_the_public_variant_and_message() {
    let error = GeneratorError::InvalidLockfile("bad graph".to_string());
    let public = crate::SbomError::from(error);

    assert_eq!(public.to_string(), "Invalid lockfile: bad graph");
    assert!(matches!(
        public,
        crate::SbomError::InvalidLockfile(message) if message == "bad graph"
    ));
}

#[test]
fn serialization_error_keeps_the_public_category_and_display_prefix() {
    let source = serde_json::from_str::<serde_json::Value>("{").unwrap_err();
    let local = GeneratorError::Serialization(source);
    assert!(std::error::Error::source(&local).is_some());
    let public = crate::SbomError::from(local);

    assert!(matches!(public, crate::SbomError::SerializationError(_)));
    assert!(public.to_string().starts_with("Serialization error:"));
    assert!(std::error::Error::source(&public).is_some());
}
