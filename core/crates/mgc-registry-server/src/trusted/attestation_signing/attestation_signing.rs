//! Creates canonical signed provenance entries and Sigstore Fulcio/Rekor attestations.
//! Tạo provenance entry có chữ ký canonical và attestation Sigstore Fulcio/Rekor.

use super::super::prelude::*;
use super::super::*;

pub const TRUSTED_PUBLISH_PREDICATE_TYPE: &str =
    "https://github.com/mingd-153/MagiCore/attestations/trusted-publish/v1";

const MAX_SIGSTORE_BUNDLE_BYTES: usize = 1024 * 1024;

/// Sign one canonical payload hash for the append-only per-package chain.
/// Ký hash payload đã chuẩn hóa cho chuỗi append-only của từng package.
pub fn sign_attestation(payload: AttestationPayload, key: &KeyPair) -> Result<SignedAttestation> {
    let canonical = serde_json::to_vec(&payload).context("serialize provenance payload")?;
    let entry_hash = hex::encode(sha2::Sha256::digest(canonical));
    let signer = key.signer().context("load registry provenance signer")?;
    let signature = signer.sign(entry_hash.as_bytes());
    Ok(SignedAttestation {
        payload,
        entry_hash,
        key_id: key.key_id.clone(),
        signature: base64::engine::general_purpose::STANDARD.encode(signature.0),
        sigstore_bundle: None,
    })
}

/// Create a Fulcio-backed in-toto attestation and record it in public Rekor.
/// Tạo in-toto attestation qua Fulcio và ghi vào Rekor công khai.
fn trusted_publish_predicate(
    package: &str,
    version: &str,
    registry_digest: &str,
    builder: &BuilderIdentity,
) -> serde_json::Value {
    serde_json::json!({
        "package": package,
        "version": version,
        "registryDigest": registry_digest,
        "builder": builder,
        "registryOutcome": "submission_attempted",
        "registryCommit": "not_asserted",
    })
}

pub async fn sign_sigstore_attestation(
    sigstore_oidc_token: &str,
    claims: &OidcClaims,
    package: &str,
    version: &str,
    subject_name: &str,
    registry_digest: &str,
    artifact_sha256: &str,
) -> Result<serde_json::Value> {
    use sigstore_sign::{Attestation, SigningContext, oidc::IdentityToken, types::Sha256Hash};

    let identity_token = IdentityToken::from_jwt(sigstore_oidc_token)
        .map_err(|_| anyhow::anyhow!("Sigstore identity token is malformed"))?;
    let digest =
        Sha256Hash::from_hex(artifact_sha256).context("parse artifact SHA-256 for Sigstore")?;
    let statement = Attestation::new(
        TRUSTED_PUBLISH_PREDICATE_TYPE,
        trusted_publish_predicate(
            package,
            version,
            registry_digest,
            &BuilderIdentity::from(claims),
        ),
    )
    .add_subject(subject_name, digest);
    let signing_context = SigningContext::production()
        .await
        .context("initialize Sigstore production signing context")?;
    let signer = signing_context.signer(identity_token);
    let bundle = tokio::time::timeout(
        std::time::Duration::from_secs(60),
        signer.sign_attestation(statement),
    )
    .await
    .map_err(|_| anyhow::anyhow!("Sigstore signing exceeded the 60-second limit"))?
    .context("Sigstore Fulcio/Rekor signing failed")?;
    let value = serde_json::to_value(bundle).context("serialize Sigstore bundle")?;
    if serde_json::to_vec(&value)?.len() > MAX_SIGSTORE_BUNDLE_BYTES {
        anyhow::bail!("Sigstore bundle exceeds the configured size limit");
    }
    Ok(value)
}

#[cfg(test)]
#[path = "test/attestation_signing.rs"]
mod tests;
