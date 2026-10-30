//! Verify an SBOM signature written by the post-build signer (SEC-15).
//! Verify chữ ký SBOM do signer post-build ghi (SEC-15).
//!
//! `mgc build` wrote `<artifact>.sig` and nothing ever read it back, so a
//! signature existed but protected nothing. Verification fails closed: an
//! untrusted key, a missing signature, a tampered artifact or a signature for a
//! different file are all errors, never a warning.
//! (`mgc build` ghi `<artifact>.sig` và không ai đọc lại, nên chữ ký tồn tại
//! mà không bảo vệ gì. Verify fail-closed: key không tin cậy, thiếu chữ ký,
//! artifact bị sửa hay chữ ký cho file khác đều là lỗi, không phải cảnh báo.)

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

/// Outcome of a successful verification.
/// Kết quả khi verify thành công.
#[derive(Debug)]
#[allow(dead_code)] // built and read by the `verify-sbom` route and its tests
pub struct Verified {
    /// Artifact that was checked.
    /// Artifact đã được kiểm.
    pub artifact: PathBuf,
    /// Absolute path of the signature used.
    /// Đường dẫn tuyệt đối của chữ ký đã dùng.
    pub signature: PathBuf,
}

/// Signature file for an artifact, following the signer's naming rule.
/// File chữ ký của artifact, theo quy tắc đặt tên của signer.
#[allow(dead_code)]
pub fn signature_path_for(artifact: &Path) -> PathBuf {
    // An extensionless artifact must become `bundle.sig`, not `bundle..sig`:
    // `with_extension` always inserts a dot. This mirrors the signer so the
    // two never disagree about where the signature lives.
    // (Artifact không có extension phải thành `bundle.sig`, không phải
    // `bundle..sig`: `with_extension` luôn chèn dấu chấm. Hàm này khớp signer
    // để hai bên không lệch chỗ đặt chữ ký.)
    let mut name = artifact.file_name().unwrap_or_default().to_os_string();
    name.push(".sig");
    artifact.with_file_name(name)
}

/// Verify `artifact` against its `.sig` file and a pinned public key.
/// Verify `artifact` với file `.sig` và public key đã pin.
#[allow(dead_code)] // reached through the `verify-sbom` route; see module docs
pub fn verify(artifact: &Path, trust_key_b64: &str) -> Result<Verified> {
    if !artifact.is_file() {
        anyhow::bail!("cannot verify '{}': not a file", artifact.display());
    }
    let signature_path = signature_path_for(artifact);
    if !signature_path.is_file() {
        anyhow::bail!(
            "no signature for '{}' (expected '{}'); a missing signature is not verified",
            artifact.display(),
            signature_path.display()
        );
    }

    let document: serde_json::Value = serde_json::from_slice(
        &std::fs::read(&signature_path)
            .with_context(|| format!("read '{}'", signature_path.display()))?,
    )
    .context("parse the signature document")?;

    let file_name = artifact
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default();
    let signed_file = document
        .get("file")
        .and_then(|value| value.as_str())
        .unwrap_or_default();
    if signed_file != file_name {
        // A signature replayed from another artifact must never validate.
        // (Chữ ký replay từ artifact khác không bao giờ được hợp lệ.)
        anyhow::bail!(
            "signature '{}' was issued for '{signed_file}', not '{file_name}'",
            signature_path.display()
        );
    }

    // The signature carries its own public key, so the caller must pin the key
    // it trusts; otherwise verification is self-attestation.
    // (Chữ ký mang public key của chính nó, nên caller phải pin key tin cậy,
    // nếu không verify chỉ là tự chứng thực.)
    let signature_key = document
        .get("public_key")
        .and_then(|value| value.as_str())
        .unwrap_or_default();
    if signature_key != trust_key_b64 {
        // Name the key that actually signed (the signature records its `key_id`)
        // so the mismatch is diagnosable against `mgc trust list`.
        // (Nêu khoá đã ký thật — chữ ký ghi `key_id`) để lỗi lệch chẩn đoán được
        // với `mgc trust list`.)
        let signed_by = document
            .get("key_id")
            .and_then(|value| value.as_str())
            .unwrap_or("an unnamed key");
        anyhow::bail!(
            "signature was made by a key that is not the pinned trust key (signed by '{signed_by}'); \
             pass that key with --trust-key, e.g. the public key `mgc trust list` prints for it"
        );
    }

    let signature = mgc_crypto::Ed25519Signature::from_base64(
        document
            .get("signature")
            .and_then(|value| value.as_str())
            .ok_or_else(|| anyhow::anyhow!("signature document has no 'signature' field"))?,
    )
    .context("decode the signature")?;
    let public_key =
        mgc_crypto::Ed25519PublicKey::from_base64(trust_key_b64).context("decode the trust key")?;
    let artifact_path = std::path::absolute(artifact)?;
    let bytes = std::fs::read(&artifact_path)
        .with_context(|| format!("read '{}'", artifact_path.display()))?;
    mgc_crypto::ed25519_signer::verify_signature(&public_key, &bytes, &signature)
        .map_err(|error| anyhow::anyhow!("signature does not verify: {error}"))?;

    Ok(Verified {
        artifact: artifact_path,
        signature: signature_path,
    })
}

#[cfg(test)]
#[path = "test/verify_sbom.rs"]
mod test;
