//! Validate statement-to-payload binding before cryptographic verification.
//! Kiểm tra statement gắn đúng payload trước khi xác minh mật mã.
#![allow(clippy::unwrap_used)]

use super::sigstore_attestation::validate_sigstore_statement;
use mgc_oidc::{OidcClaims, claims::Audience};
use mgc_registry_server::trusted::{AttestationPayload, BuilderIdentity};
use sigstore_types::Statement;

fn payload() -> AttestationPayload {
    AttestationPayload {
        schema_version: 1,
        package: "pkg".into(),
        version: "1.0.0".into(),
        digest: "sha512-example".into(),
        artifact_sha256: Some(hex::encode([7u8; 32])),
        sequence: 1,
        builder: BuilderIdentity::from(&OidcClaims {
            iss: "https://token.actions.githubusercontent.com".into(),
            sub: "repo:acme/pkg:ref:refs/heads/main".into(),
            aud: Audience::Single("registry".into()),
            exp: 2,
            iat: 1,
            repository: Some("acme/pkg".into()),
            job_workflow_ref: None,
            workflow_ref: None,
            event_name: None,
        }),
        created_at_unix: 1,
        previous_hash: None,
    }
}

fn statement(payload: &AttestationPayload, digest: &str) -> Statement {
    serde_json::from_value(serde_json::json!({
        "_type": "https://in-toto.io/Statement/v1",
        "subject": [{"name": "pkg-1.0.0", "digest": {"sha256": digest}}],
        "predicateType": mgc_registry_server::trusted::TRUSTED_PUBLISH_PREDICATE_TYPE,
        "predicate": {
            "package": payload.package,
            "version": payload.version,
            "registryDigest": payload.digest,
            "builder": serde_json::to_value(&payload.builder).unwrap(),
        }
    }))
    .unwrap()
}

#[test]
fn accepts_a_statement_bound_to_the_payload() {
    let payload = payload();
    let statement = statement(&payload, payload.artifact_sha256.as_deref().unwrap());
    assert!(validate_sigstore_statement(&payload, &statement).is_ok());
}

#[test]
fn rejects_a_statement_with_a_different_artifact_digest() {
    let payload = payload();
    let statement = statement(&payload, &hex::encode([8u8; 32]));
    assert!(validate_sigstore_statement(&payload, &statement).is_err());
}
