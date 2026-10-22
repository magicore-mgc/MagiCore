//! Installed metadata error contract regression.
//! Test hồi quy contract lỗi metadata đã cài.

use super::*;

#[test]
fn invalid_metadata_is_typed_and_keeps_the_existing_message() {
    let error = registry_package_invalid_bins();
    assert_eq!(
        error.to_string(),
        "package runner: installed package identity or executable metadata is invalid"
    );
    assert!(matches!(
        error.downcast_ref::<InstalledMetadataError>(),
        Some(InstalledMetadataError::InvalidPackageMetadata)
    ));
}
