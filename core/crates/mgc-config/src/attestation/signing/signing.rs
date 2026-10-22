//! Attestation signing and storage / Ký và lưu attestation.

use std::path::Path;

use crate::attestation::{Attestation, PreviousKey, canonical_message};

fn now_rfc3339() -> String {
    time::OffsetDateTime::now_utc()
        .format(&time::format_description::well_known::Rfc3339)
        .unwrap_or_default()
}

/// Attest version tag — the mgc build that attested (compile-time).
/// (Version mgc đã chứng thực — compile-time.)
pub fn attester_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
#[allow(clippy::too_many_arguments)]
pub(in crate::attestation) fn sign_and_store(
    project_root: &Path,
    core: &str,
    digest: &str,
    key_pair: &mgc_crypto::keyring::KeyPair,
    prev_keys: Vec<PreviousKey>,
    slot: &Path,
    path: &Path,
) -> anyhow::Result<Attestation> {
    let signer = key_pair
        .signer()
        .map_err(|error| anyhow::anyhow!("cannot use key: {error}"))?;
    let signature = signer.sign(canonical_message(core, digest).as_bytes());
    let attestation = Attestation {
        core: core.to_string(),
        config_digest: digest.to_string(),
        project_path: project_root.to_string_lossy().to_string(),
        key_id: key_pair.key_id.clone(),
        public_key: key_pair.public_key.to_base64(),
        prev_keys,
        attested_at: now_rfc3339(),
        mgc_version: attester_version().to_string(),
        signature: signature.to_base64(),
    };
    std::fs::create_dir_all(slot)
        .map_err(|error| anyhow::anyhow!("cannot create attestation slot: {error}"))?;
    let text = serde_json::to_string_pretty(&attestation)
        .map_err(|error| anyhow::anyhow!("cannot serialize attestation: {error}"))?;
    std::fs::write(path, text)
        .map_err(|error| anyhow::anyhow!("cannot write attestation: {error}"))?;
    Ok(attestation)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
