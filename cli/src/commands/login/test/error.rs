//! Test stable login diagnostic wording.
//! Kiểm tra nội dung diagnostic login ổn định.
#[test]
fn username_error_keeps_the_existing_english_diagnostic() {
    assert_eq!(
        super::LoginError::UsernameRequired
            .into_diagnostic()
            .to_string(),
        "username is required"
    );
}

#[test]
fn local_errors_keep_legacy_registry_diagnostics() {
    use super::super::credentials::CredentialInputError;
    use super::super::registry_user_creation::RegistryUserCreationError;
    use super::super::token_storage::TokenStorageError;

    assert_eq!(
        super::super::credentials::credential_input_diagnostic(CredentialInputError::Empty)
            .to_string(),
        crate::error::registry_auth_password_required().to_string()
    );
    assert_eq!(
        super::super::credentials::credential_input_diagnostic(CredentialInputError::TooLarge)
            .to_string(),
        crate::error::registry_auth_password_too_large().to_string()
    );
    assert_eq!(
        super::super::registry_user_creation::registration_diagnostic(
            RegistryUserCreationError::InvalidUrl
        )
        .to_string(),
        crate::error::registry_auth_url_invalid().to_string()
    );
    assert_eq!(
        super::super::registry_user_creation::registration_diagnostic(
            RegistryUserCreationError::RequiresHttps
        )
        .to_string(),
        crate::error::registry_auth_requires_https().to_string()
    );
    assert_eq!(
        super::super::registry_user_creation::registration_diagnostic(
            RegistryUserCreationError::RequestFailed
        )
        .to_string(),
        crate::error::registry_auth_request_failed().to_string()
    );
    assert_eq!(
        super::super::registry_user_creation::registration_diagnostic(
            RegistryUserCreationError::StatusFailed(401)
        )
        .to_string(),
        crate::error::registry_auth_status_failed(401).to_string()
    );
    assert_eq!(
        super::super::registry_user_creation::registration_diagnostic(
            RegistryUserCreationError::ResponseTooLarge
        )
        .to_string(),
        crate::error::registry_auth_response_too_large().to_string()
    );
    assert_eq!(
        super::super::registry_user_creation::registration_diagnostic(
            RegistryUserCreationError::ResponseInvalid
        )
        .to_string(),
        crate::error::registry_auth_response_invalid().to_string()
    );
    assert_eq!(
        super::super::registry_user_creation::registration_diagnostic(
            RegistryUserCreationError::TokenMissing
        )
        .to_string(),
        crate::error::no_token_in_response().to_string()
    );
    assert_eq!(
        super::super::token_storage::token_storage_diagnostic(TokenStorageError::NoHomeDirectory)
            .to_string(),
        crate::error::no_home_dir().to_string()
    );
}

#[test]
fn source_errors_keep_the_original_anyhow_chain() {
    use anyhow::Context;

    let original = anyhow::anyhow!("registry transport failed").context("send registration");
    let expected = format!("{original:#}");
    let actual = super::LoginError::Source(original).into_diagnostic();
    assert_eq!(format!("{actual:#}"), expected);
}
