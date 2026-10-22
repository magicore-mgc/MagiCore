//! Verify the detached manifest signature against trusted keys.
//! Xác minh chữ ký manifest rời bằng khóa tin cậy.

use crate::commands::self_update::error::SelfUpdateError;
use crate::commands::self_update::{MAX_DOC_BYTES, ProvenanceMode, download_capped, trust_roots};
use anyhow::Result;
/// Verify the detached manifest signature against trust roots. Verified
/// mode fails closed on ANY signature problem (missing or invalid);
/// only the explicit unsigned opt-in (no roots) proceeds sha256-only,
/// LOUDLY. A PRESENT but invalid signature always fails, keys or not.
/// (Verify chữ ký — Verified fail-closed mọi lỗi; chỉ opt-in tường minh
/// mới đi tiếp sha256-only.)
pub(in crate::commands::self_update::manifest) async fn verify_manifest_signature(
    manifest_sig_url: &str,
    manifest_bytes: &[u8],
    trust_root: &[String],
    mode: ProvenanceMode,
) -> Result<()> {
    let roots = trust_roots(trust_root);
    let sig_bytes = match download_capped(manifest_sig_url, MAX_DOC_BYTES).await {
        Ok(bytes) => bytes,
        Err(e) if mode == ProvenanceMode::Verified => {
            return Err(SelfUpdateError::signature_invalid(&format!(
                "trust roots configured but signature download failed: {e}"
            )));
        }
        Err(_) => {
            mgc_ui::warning(
                "no manifest signature published and no trust roots configured — provenance UNVERIFIED (sha256 transport check only; explicit --allow-unsigned)",
            );
            return Ok(());
        }
    };
    if roots.is_empty() {
        if mode == ProvenanceMode::Verified {
            return Err(SelfUpdateError::signature_invalid(
                "manifest signature present but trust-root resolution is inconsistent — refusing unsigned verification",
            ));
        }
        mgc_ui::warning(
            "manifest signature present but no trust roots configured (--trust-root or MGC_RELEASE_TRUST_ROOTS) — provenance UNVERIFIED (explicit --allow-unsigned)",
        );
        return Ok(());
    }
    let sig_text = String::from_utf8(sig_bytes)
        .map_err(|e| SelfUpdateError::signature_invalid(&e.to_string()))?;
    let sig_hex = sig_text
        .split_whitespace()
        .next()
        .ok_or_else(|| SelfUpdateError::signature_invalid("empty signature file"))?;
    let sig_bytes =
        hex::decode(sig_hex).map_err(|e| SelfUpdateError::signature_invalid(&e.to_string()))?;
    let signature = mgc_crypto::ed25519_signer::Ed25519Signature(sig_bytes);
    for root in &roots {
        let key_bytes = hex::decode(root.trim())
            .map_err(|e| SelfUpdateError::signature_invalid(&format!("bad trust-root hex: {e}")))?;
        let key = mgc_crypto::ed25519_signer::Ed25519PublicKey(key_bytes);
        if mgc_crypto::ed25519_signer::verify_signature(&key, manifest_bytes, &signature).is_ok() {
            return Ok(());
        }
    }
    Err(SelfUpdateError::signature_invalid(
        "no configured trust root verifies this manifest",
    ))
}
