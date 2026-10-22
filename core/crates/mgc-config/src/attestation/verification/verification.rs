//! Project attestation verification / Xác minh attestation project.

use std::path::Path;

use crate::attestation::{
    ATTESTATION_FILE, Attestation, AttestationStatus, attestation_dir, blake3_hex,
    canonical_message, read_config_bytes,
};

/// Verify the live project against its attestation. Verification needs
/// no secrets: the attestation embeds every public key it trusts, so a
/// machine holding no private key can still check integrity.
/// (Verify project hiện tại với chứng thực của nó. Verify không cần
/// secret: chứng thực nhúng mọi public key nó tin.)
pub fn verify(project_root: &Path, identities_dir: &Path) -> anyhow::Result<AttestationStatus> {
    let path = attestation_dir(identities_dir, project_root).join(ATTESTATION_FILE);
    let text = match std::fs::read_to_string(&path) {
        Ok(text) => text,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok(AttestationStatus::Absent);
        }
        Err(error) => return Err(anyhow::anyhow!("cannot read attestation: {error}")),
    };
    let attestation: Attestation = serde_json::from_str(&text)
        .map_err(|error| anyhow::anyhow!("attestation is not valid JSON: {error}"))?;
    let live_core = match crate::project::ProjectConfig::read_core_marker_identity(project_root) {
        Ok(Some(core)) => core,
        Ok(None) => {
            return Ok(AttestationStatus::Mismatch(
                "project core marker is missing".to_string(),
            ));
        }
        Err(error) => {
            return Ok(AttestationStatus::Mismatch(format!(
                "project core marker is invalid: {error}"
            )));
        }
    };
    if live_core != attestation.core {
        return Ok(AttestationStatus::Mismatch(format!(
            "live core marker is '{live_core}', but the attestation binds '{}'",
            attestation.core
        )));
    }
    let config = match read_config_bytes(project_root) {
        Ok(config) => config,
        Err(_) => {
            return Ok(AttestationStatus::Mismatch(
                "project mgc.toml is missing or unreadable".to_string(),
            ));
        }
    };
    let digest = blake3_hex(&config);
    if digest != attestation.config_digest {
        return Ok(AttestationStatus::Mismatch(
            "mgc.toml changed since attestation (re-attest after intentional edits)".to_string(),
        ));
    }
    let message = canonical_message(&attestation.core, &attestation.config_digest);
    let mut candidates = vec![attestation.public_key.clone()];
    for previous in &attestation.prev_keys {
        candidates.push(previous.public_key.clone());
    }
    for public_key_b64 in candidates {
        let Ok(public_key) =
            mgc_crypto::ed25519_signer::Ed25519PublicKey::from_base64(&public_key_b64)
        else {
            continue;
        };
        let Ok(signature) =
            mgc_crypto::ed25519_signer::Ed25519Signature::from_base64(&attestation.signature)
        else {
            continue;
        };
        if mgc_crypto::ed25519_signer::verify_signature(&public_key, message.as_bytes(), &signature)
            .is_ok()
        {
            return Ok(AttestationStatus::Valid);
        }
    }
    Ok(AttestationStatus::Mismatch(
        "signature does not verify against the attested keys".to_string(),
    ))
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
