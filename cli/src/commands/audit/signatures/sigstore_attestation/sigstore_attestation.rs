//! Verify public artifact attestations against the trusted builder identity and statement.
//! Xác minh attestation artifact theo builder tin cậy và statement.

use super::error::SigstoreAttestationError;
use sigstore_types::{Bundle as SigstoreBundle, Sha256Hash, SignatureContent, Statement};
use sigstore_verify::{VerificationPolicy, Verifier};
use std::time::Duration;

const TRUST_ROOT_TIMEOUT_SECS: u64 = 60;

/// Load the production trust root once for the paginated audit.
/// Tải trust root production một lần cho lượt audit phân trang.
/// Release workflows allowed to attest artifacts.
/// Workflow release được phép chứng thực artifact.
const DEFAULT_WORKFLOW_IDENTITY_PREFIX: &str = "https://github.com/";

fn trusted_workflow_identities() -> Vec<String> {
    std::env::var("MAGICORE_SIGSTORE_WORKFLOW_ALLOWLIST")
        .ok()
        .map(|value| {
            value
                .split(',')
                .map(str::trim)
                .filter(|item| !item.is_empty())
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default()
}

pub(in crate::commands::audit::signatures) async fn production_verifier()
-> std::result::Result<Verifier, SigstoreAttestationError> {
    let trusted_root = tokio::time::timeout(
        Duration::from_secs(TRUST_ROOT_TIMEOUT_SECS),
        sigstore_trust_root::TrustedRoot::production(),
    )
    .await
    .map_err(|_| {
        SigstoreAttestationError::External(anyhow::anyhow!("Sigstore trust-root fetch timed out"))
    })?
    .map_err(|error| SigstoreAttestationError::External(error.into()))?;
    Verifier::new(&trusted_root).map_err(|_| SigstoreAttestationError::Invalid)
}

pub(in crate::commands::audit::signatures) fn verify_public_attestation(
    entry: &mgc_registry_server::trusted::SignedAttestation,
    bundle_json: &serde_json::Value,
    verifier: &Verifier,
) -> std::result::Result<(), SigstoreAttestationError> {
    let encoded_bundle = serde_json::to_string(bundle_json)
        .map_err(|error| SigstoreAttestationError::External(error.into()))?;
    let bundle = SigstoreBundle::from_json(&encoded_bundle)
        .map_err(|_| SigstoreAttestationError::Invalid)?;
    let SignatureContent::DsseEnvelope(envelope) = &bundle.content else {
        return Err(SigstoreAttestationError::Invalid);
    };
    if envelope.payload_type != "application/vnd.in-toto+json" {
        return Err(SigstoreAttestationError::Invalid);
    }
    let statement: Statement = serde_json::from_slice(envelope.payload.as_bytes())
        .map_err(|_| SigstoreAttestationError::Invalid)?;
    validate_sigstore_statement(&entry.payload, &statement)?;
    let artifact_hash = entry
        .payload
        .artifact_sha256
        .as_deref()
        .filter(|digest| mgc_registry_server::trusted::is_valid_sha256_hex(digest))
        .ok_or(SigstoreAttestationError::Invalid)?;
    let workflow = entry
        .payload
        .builder
        .job_workflow_ref
        .as_deref()
        .or(entry.payload.builder.workflow_ref.as_deref())
        .ok_or(SigstoreAttestationError::Invalid)?;
    // SEC-05: the workflow name comes from the payload under verification, so
    // trusting it is self-attestation — anyone can sign with any workflow ref.
    // The identity must match a pinned allowlist instead.
    // (SEC-05: tên workflow lấy từ chính payload đang verify nên tin nó là tự
    // chứng thực — ai cũng ký được với workflow ref bất kỳ. Identity phải khớp
    // allowlist đã pin.)
    let workflow_identity = format!("https://github.com/{workflow}");
    let allowed = trusted_workflow_identities();
    if !allowed.is_empty() && !allowed.contains(&workflow_identity) {
        return Err(SigstoreAttestationError::Invalid);
    }
    if allowed.is_empty() {
        // Fail closed: without a pinned allowlist the only acceptable shape is
        // this repository's own release workflow.
        // (Fail-closed: không có allowlist pin thì dạng duy nhất được chấp nhận
        // là workflow release của chính repo này.)
        if !(workflow_identity.starts_with(DEFAULT_WORKFLOW_IDENTITY_PREFIX)
            && workflow_identity.contains("/.github/workflows/release"))
        {
            return Err(SigstoreAttestationError::Invalid);
        }
    }
    let expected_identity = workflow_identity;
    let policy = VerificationPolicy::new(expected_identity, entry.payload.builder.issuer.clone());
    verifier
        .verify(
            Sha256Hash::from_hex(artifact_hash)
                .map_err(|error| SigstoreAttestationError::External(error.into()))?,
            &bundle,
            &policy,
        )
        .map_err(|_| SigstoreAttestationError::Invalid)?;
    Ok(())
}

pub(in crate::commands::audit::signatures) fn validate_sigstore_statement(
    payload: &mgc_registry_server::trusted::AttestationPayload,
    statement: &Statement,
) -> std::result::Result<(), SigstoreAttestationError> {
    let artifact_sha256 = payload
        .artifact_sha256
        .as_deref()
        .filter(|digest| mgc_registry_server::trusted::is_valid_sha256_hex(digest))
        .ok_or(SigstoreAttestationError::Invalid)?;
    let expected_builder = serde_json::to_value(&payload.builder)
        .map_err(|error| SigstoreAttestationError::External(error.into()))?;
    let predicate = &statement.predicate;
    let subject_matches = statement.subject.iter().any(|subject| {
        !subject.name.is_empty()
            && subject
                .digest
                .sha256
                .as_ref()
                .is_some_and(|digest| digest.to_hex() == artifact_sha256)
    });
    if statement.type_ != "https://in-toto.io/Statement/v1"
        || statement.predicate_type != mgc_registry_server::trusted::TRUSTED_PUBLISH_PREDICATE_TYPE
        || !subject_matches
        || predicate.get("package").and_then(serde_json::Value::as_str)
            != Some(payload.package.as_str())
        || predicate.get("version").and_then(serde_json::Value::as_str)
            != Some(payload.version.as_str())
        || predicate
            .get("registryDigest")
            .and_then(serde_json::Value::as_str)
            != Some(payload.digest.as_str())
        || predicate.get("builder") != Some(&expected_builder)
    {
        return Err(SigstoreAttestationError::Invalid);
    }
    Ok(())
}
