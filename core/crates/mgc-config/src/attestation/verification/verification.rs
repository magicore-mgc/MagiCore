//! Project attestation verification / Xác minh attestation project.

use std::path::Path;

use crate::attestation::{
    ATTESTATION_FILE, Attestation, AttestationStatus, attestation_dir, blake3_hex,
    canonical_message, read_config_bytes, signed_message,
};

/// Verify the live project against its attestation.
///
/// The attestation itself cannot be the trust root (SIG-F7/SEC-12): it carries
/// the public key it is verified with, so anyone able to rewrite the file can
/// re-sign it. A key from the file is accepted only when the project also
/// pins it in `mgc.toml` under `[trust].keys`, or when the operator names it
/// in `MAGICORE_TRUST_KEYS`. Otherwise the status is `Mismatch`.
/// (Chứng thực không thể là gốc tin cậy: file mang chính public key dùng để
/// verify, nên ai ghi lại file là ký lại được. Key trong file chỉ được chấp
/// nhận khi project pin nó trong `mgc.toml` mục `[trust].keys`, hoặc khi
/// operator chỉ định qua `MAGICORE_TRUST_KEYS`. Ngược lại trả `Mismatch`.)
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
    // P-8.1/P-8.2: when a sibling signature exists, it must cover the exact
    // bytes on disk before the document's own fields are trusted.
    // (P-8.1/P-8.2: khi có chữ ký cạnh file, nó phải phủ đúng bytes trên đĩa
    // trước khi tin các field của tài liệu.)
    let mut sidecar = path.as_os_str().to_os_string();
    sidecar.push(".sig");
    let sidecar = std::path::PathBuf::from(sidecar);
    if let Ok(raw) = std::fs::read(&path)
        && sidecar.is_file()
    {
        let trusted = trusted_keys(project_root);
        if !trusted.is_empty() {
            let signature_text = std::fs::read_to_string(&sidecar)
                .map_err(|error| anyhow::anyhow!("cannot read the attestation sidecar: {error}"))?;
            let signature = mgc_crypto::Ed25519Signature::from_base64(signature_text.trim())
                .map_err(|error| anyhow::anyhow!("sidecar signature is not base64: {error}"))?;
            let mut verified = false;
            for key in &trusted {
                if let Ok(public_key) = mgc_crypto::Ed25519PublicKey::from_base64(key)
                    && mgc_crypto::ed25519_signer::verify_signature(&public_key, &raw, &signature)
                        .is_ok()
                {
                    verified = true;
                    break;
                }
            }
            if !verified {
                return Ok(AttestationStatus::Mismatch(
                    "attestation file was modified after it was signed (sidecar mismatch)"
                        .to_string(),
                ));
            }
        }
    }
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
    // Verify the identity-bound message when the attestation carries one, and
    // report a domain mismatch separately so a signature copied from another
    // core is distinguishable from a corrupt signature (P-8.3).
    // (Verify message có ràng buộc identity khi attestation có; báo domain
    // mismatch riêng để chữ ký copy từ core khác phân biệt được với chữ ký
    // hỏng — P-8.3.)
    let identity_mismatch = attestation.core_identity.as_ref().is_some_and(|identity| {
        identity
            .ecosystems
            .iter()
            .any(|ecosystem| ecosystem != &attestation.core)
    });
    if identity_mismatch {
        return Ok(AttestationStatus::DomainMismatch(
            "signature was issued for a different core identity".to_string(),
        ));
    }
    let message = match attestation.core_identity.as_ref() {
        Some(identity) => signed_message(&attestation.core, &attestation.config_digest, identity),
        None => canonical_message(&attestation.core, &attestation.config_digest),
    };
    let mut candidates = vec![attestation.public_key.clone()];
    // P-8.5: a rotated-out key is only accepted inside its grace window.
    // (P-8.5: key bị rotation chỉ được chấp nhận trong thời gian ân hạn.)
    let now = crate::attestation::rotation::now_rfc3339();
    for previous in &attestation.prev_keys {
        let expired = previous.expires_at.as_deref().is_some_and(|deadline| {
            crate::attestation::rotation::rotation_key_expired(deadline, &now)
        });
        if expired {
            return Ok(AttestationStatus::Mismatch(format!(
                "previous key '{}' passed its rotation grace deadline and is no longer accepted",
                previous.key_id
            )));
        }
        candidates.push(previous.public_key.clone());
    }
    let trusted = trusted_keys(project_root);
    if !trusted.is_empty() {
        candidates.retain(|key| trusted.iter().any(|pinned| pinned == key));
        if candidates.is_empty() {
            return Ok(AttestationStatus::Mismatch(
                "attestation key is not one of the trusted keys pinned for this project"
                    .to_string(),
            ));
        }
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

/// Keys the project or the operator explicitly trusts (SIG-F7).
/// Key được project hoặc operator tin cậy tường minh (SIG-F7).
fn trusted_keys(project_root: &Path) -> Vec<String> {
    let mut keys = Vec::new();
    if let Ok(Some(config)) = crate::project::ProjectConfig::load(project_root)
        && let Some(trust) = config.trust
    {
        keys.extend(trust.keys);
    }
    if let Ok(list) = std::env::var("MAGICORE_TRUST_KEYS") {
        keys.extend(
            list.split(',')
                .map(str::trim)
                .filter(|item| !item.is_empty())
                .map(str::to_string),
        );
    }
    keys
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
