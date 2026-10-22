//! Display tests for `ScaffoldSpecError`.
//! Test display cho `ScaffoldSpecError`.

use super::super::ScaffoldSpecError;

#[test]
fn preserves_every_existing_spec_validation_message() {
    let cases = [
        (
            ScaffoldSpecError::TemplateNameEmpty,
            "Template name cannot be empty",
        ),
        (ScaffoldSpecError::EmptyDistTag, "Empty dist tag"),
        (
            ScaffoldSpecError::UnknownTagWithSuggestion {
                tag: "laster".to_string(),
                suggestion: "latest".to_string(),
            },
            "Unknown tag 'laster'. Did you mean 'latest'?",
        ),
        (ScaffoldSpecError::EmptyVersion, "Empty version"),
        (
            ScaffoldSpecError::VersionMustStartWithDigit {
                version: "latest".to_string(),
            },
            "Version must start with digit, got 'latest'",
        ),
        (
            ScaffoldSpecError::VersionRangesUnsupported,
            "Version ranges not supported yet",
        ),
    ];

    for (error, expected) in cases {
        assert_eq!(error.to_string(), expected);
    }
}

#[test]
fn converts_typed_error_to_the_legacy_anyhow_message_form() {
    let error = ScaffoldSpecError::VersionRangesUnsupported.into_anyhow();

    assert_eq!(error.to_string(), "Version ranges not supported yet");
    assert_eq!(error.chain().count(), 1);
    assert!(error.downcast_ref::<ScaffoldSpecError>().is_none());
}
