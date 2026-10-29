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
//! The key comes from the data directory that `mgc store` already manages;
//! a project without a keyring is not signed and says why, because silently
//! producing unsigned artefacts is how "signed releases" become fiction.
//! Khoá đến từ data dir mà `mgc store` đã quản lý; project chưa có keyring thì
//! không ký và nói rõ lý do, vì lặng lẽ tạo artefact không ký là cách "bản phát
//! hành có ký" trở thành hư cấu.

use std::path::Path;

use anyhow::Result;

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

/// Sign `artifact` with the project keyring, writing `<artifact>.sig`.
/// Ký `artifact` bằng keyring của project, ghi `<artifact>.sig`.
pub(crate) fn sign_artifact(artifact: &Path, keyring_path: &Path) -> Result<Signed> {
    if !artifact.is_file() {
        anyhow::bail!("cannot sign '{}': not a file", artifact.display());
    }
    let bytes = std::fs::read(artifact)?;

    let signer = match load_signer(keyring_path) {
        Ok(signer) => signer,
        Err(error) => {
            return Ok(Signed::Skipped(format!(
                // The message used to say "run `mgc key generate`" — there is no
                // such command. What the loader actually needs is a keyring JSON
                // file with a `signing_key` field holding a 32-byte hex seed, so
                // the message names that shape instead of a command that cannot
                // be typed.
                // (Message từng nói "run `mgc key generate`" — lệnh không tồn tại.
                // Loader cần file keyring JSON có field `signing_key` chứa seed hex
                // 32 byte, nên message nêu đúng hình dạng đó.)
                "no signing key at '{}': {error}; create that file with a JSON object \
                 like {{\"signing_key\": \"<64 hex chars = 32-byte Ed25519 seed>\"}}",
                keyring_path.display()
            )));
        }
    };

    let signature = signer.sign(&bytes);
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
        "signature": encoded,
        "public_key": signer.public_key().to_base64(),
    });
    std::fs::write(&signature_path, serde_json::to_vec_pretty(&document)?)?;
    Ok(Signed::Signature(signature_path))
}

type Signer = mgc_crypto::Ed25519Signer;

fn load_signer(keyring_path: &Path) -> Result<Signer> {
    let raw = std::fs::read(keyring_path)?;
    let value: serde_json::Value = serde_json::from_slice(&raw)?;
    let seed_hex = value
        .get("signing_key")
        .and_then(|k| k.as_str())
        .ok_or_else(|| anyhow::anyhow!("keyring has no 'signing_key' field"))?;
    let seed = decode_hex(seed_hex)?;
    if seed.len() != 32 {
        anyhow::bail!("signing key must be 32 bytes, got {}", seed.len());
    }
    let mut array = [0u8; 32];
    array.copy_from_slice(&seed);
    Ok(mgc_crypto::Ed25519Signer::from_seed(&array)?)
}

fn decode_hex(value: &str) -> Result<Vec<u8>> {
    if !value.len().is_multiple_of(2) {
        anyhow::bail!("hex string has odd length");
    }
    let mut out = Vec::with_capacity(value.len() / 2);
    let bytes = value.as_bytes();
    for pair in bytes.chunks(2) {
        let text = std::str::from_utf8(pair)?;
        out.push(u8::from_str_radix(text, 16)?);
    }
    Ok(out)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
