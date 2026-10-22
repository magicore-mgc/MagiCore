//! Stable CLI diagnostics for signature-audit coordinator errors.
//! Diagnostic CLI ổn định cho lỗi điều phối audit chữ ký.

use super::super::error::SignatureAuditError;

#[test]
fn maps_typed_coordinator_errors_to_existing_messages() {
    assert_eq!(
        SignatureAuditError::Invalid.into_cli().to_string(),
        "trusted provenance signature or transparency chain verification failed"
    );
    assert_eq!(
        SignatureAuditError::MissingOrMismatched
            .into_cli()
            .to_string(),
        "registry returned no provenance entries or a package mismatch"
    );
    assert_eq!(
        SignatureAuditError::RegistryUrlInvalid
            .into_cli()
            .to_string(),
        "trusted publishing requires a valid registry URL without credentials or fragment"
    );
    assert_eq!(
        SignatureAuditError::RequiresHttps.into_cli().to_string(),
        "trusted publishing requires HTTPS except for loopback registry URLs"
    );
    assert_eq!(
        SignatureAuditError::RequestFailed(503)
            .into_cli()
            .to_string(),
        "trusted signatures request failed with HTTP 503"
    );
}
