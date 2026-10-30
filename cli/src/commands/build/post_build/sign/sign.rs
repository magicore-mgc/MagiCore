//! Sign the output SBOM so a deployed bundle can be proven.
//! Ký SBOM output để chứng minh được bundle đã deploy.
//!
//! A hash list that anyone can rewrite proves nothing. Signing the SBOM turns
//! it into a statement: this build produced these files, and this key says so.
//! The signature ships beside the artefact, so a server or a CI gate can verify
//! it without trusting the machine that produced it.
//! Danh sách hash mà ai cũng sửa được thì không chứng minh gì. Ký SBOM biến nó
//! thành một phát biểu: bản build này tạo ra những file này, và khoá này xác
//! nhận. Chữ ký đi kèm artefact, nên server hoặc cổng CI verify được mà không
//! phải tin máy đã tạo ra nó.
//!
//! The key is the default key of the `mgc trust` keyring — the same key
//! `mgc trust sign` uses — so "I ran `mgc trust init`" actually means builds are
//! signed. A project without a keyring is not signed and says why, because
//! silently producing unsigned artefacts is how "signed releases" become fiction.
//! Khoá là khoá mặc định của keyring `mgc trust` — đúng khoá `mgc trust sign`
//! dùng — nên "tôi đã chạy `mgc trust init`" thực sự có nghĩa build được ký.
//! Project không có keyring thì không ký và nói rõ lý do, vì lặng lẽ tạo
//! artefact không ký là cách "bản phát hành có ký" trở thành hư cấu.

use std::path::Path;

use anyhow::Result;
use mgc_ui::warning;

use super::key_source::resolve_signing_key;

/// Outcome of an attempted signing.
/// Kết quả của một lần thử ký.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Signed {
    /// A detached signature was written to the returned path.
    /// Chữ ký rời đã ghi vào đường dẫn trả về.
    Signature(std::path::PathBuf),
    /// Nothing was signed, with the reason.
    /// Không ký gì, kèm lý do.
    Skipped(String),
}

/// Sign `artifact` with the signed-in keyring, writing `<artifact>.sig`.
///
/// `data_dir` is the MagiCore data directory; it only matters for the legacy
/// single-seed keyring, which is still honoured after a warning.
/// Ký `artifact` bằng keyring đã đăng ký, ghi `<artifact>.sig`. `data_dir` là
/// data dir MagiCore; chỉ dùng cho keyring một-seed kiểu cũ, vẫn được tôn trọng
/// kèm cảnh báo.
pub(crate) fn sign_artifact(artifact: &Path, data_dir: &Path) -> Result<Signed> {
    if !artifact.is_file() {
        anyhow::bail!("cannot sign '{}': not a file", artifact.display());
    }
    let bytes = std::fs::read(artifact)?;

    let key = match resolve_signing_key(data_dir) {
        Ok(key) => key,
        Err(error) => {
            if let Some(reason) = error.downcast_ref::<super::key_source::KeyResolutionError>()
                && matches!(reason, super::key_source::KeyResolutionError::NoKey)
            {
                return Ok(Signed::Skipped(error.to_string()));
            }
            // A keyring that exists but cannot be read is a broken signing
            // setup, not a reason to publish silently unsigned. Fail closed so
            // the user fixes the key they believe is in effect.
            // (Keyring có tồn tại nhưng không đọc được là thiết lập ký hỏng,
            // không phải lý do phát hành im lặng không ký. Fail-closed để người
            // dùng sửa đúng khoá mà họ tưởng đang dùng.)
            return Err(error);
        }
    };

    if let Some(other) = &key.conflict {
        // Two keyrings exist: say which one signed, because a verifier pins a
        // public key and a mismatch is the user's next support question.
        // (Có hai keyring: nói rõ cái nào đã ký, vì verifier pin public key và
        // lệch khoá là câu hỏi tiếp theo của người dùng.)
        warning(&format!(
            "two signing keyrings exist; signing with '{}' (key {}). Ignoring '{}'; remove or \
             consolidate it, or set {} to choose explicitly",
            key.source.display(),
            key.key_id,
            other.display(),
            super::key_source::SIGNING_KEYRING_ENV,
        ));
    }

    let signature = key.signer.sign(&bytes);
    let encoded = signature.to_base64();
    // SEC-15: one naming rule shared with the verifier. `with_extension`
    // produced `bundle..sig` for an extensionless artifact, which the verifier
    // would never look for.
    // (SEC-15: một quy tắc đặt tên dùng chung với verifier. `with_extension`
    // tạo `bundle..sig` cho artifact không có extension, verifier sẽ không bao
    // giờ tìm file đó.)
    let mut signed_name = artifact.file_name().unwrap_or_default().to_os_string();
    signed_name.push(".sig");
    let signature_path = artifact.with_file_name(signed_name);
    let document = serde_json::json!({
        "file": artifact.file_name().and_then(|n| n.to_str()).unwrap_or_default(),
        "algorithm": "ed25519",
        // The key that signed, so `mgc verify-sbom --trust-key` can be pinned
        // from the same keyring (`mgc trust list` prints it).
        // (Khoá đã ký, để `mgc verify-sbom --trust-key` pin được từ chính keyring
        // — `mgc trust list` in ra.)
        "key_id": key.key_id,
        "signature": encoded,
        "public_key": key.public_key_b64,
    });
    std::fs::write(&signature_path, serde_json::to_vec_pretty(&document)?)?;
    Ok(Signed::Signature(signature_path))
}

#[cfg(test)]
#[path = "test/mod.rs"]
pub(crate) mod tests;
