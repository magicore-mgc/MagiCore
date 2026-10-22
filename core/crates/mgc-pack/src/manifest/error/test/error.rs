//! Tests for stable manifest sanitization diagnostics.
//! Test diagnostic sanitize manifest ổn định.

use super::ManifestSanitizeError;

#[test]
fn validation_errors_keep_the_existing_display_text() {
    let cases = [
        (
            ManifestSanitizeError::NotJsonObject,
            "manifest is not a JSON object",
        ),
        (
            ManifestSanitizeError::PrivatePackage,
            "package.json has private: true — cannot publish (remove private: true to publish)",
        ),
        (
            ManifestSanitizeError::InvalidPrivateType,
            "package.json private must be a boolean",
        ),
        (
            ManifestSanitizeError::MissingName,
            "package.json missing name",
        ),
        (
            ManifestSanitizeError::MissingVersion,
            "package.json missing version",
        ),
    ];

    for (error, expected) in cases {
        assert_eq!(error.to_string(), expected);
    }
}
