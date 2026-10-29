//! Signed binary manifests for the lib core transfer layer (E-3).
//! Manifest binary có chữ ký cho lớp luân chuyển binary của core lib (E-3).
//!
//! A manifest lists downloadable binaries with their URL and BLAKE3 digest and
//! is signed with Ed25519 by a key the user pins. Nothing is downloaded or
//! installed unless the signature verifies against a trusted key and the
//! downloaded bytes match the digest recorded in the signed document.
//! (Manifest liệt kê binary kèm URL và digest BLAKE3, được ký Ed25519 bằng key
//! người dùng pin. Không tải hoặc cài gì trừ khi chữ ký verify với key tin cậy
//! và bytes tải về khớp digest ghi trong tài liệu đã ký.)

use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::install::keg::error::{KegError, KegResult};

/// Domain label bound into a manifest signature.
/// Nhãn domain ràng buộc trong chữ ký manifest.
pub const MANIFEST_DOMAIN: &str = "mgc-lib-binary-manifest/v1";

/// One binary entry.
/// Một mục binary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BinaryEntry {
    /// Binary (command) name, used for the keg and the PATH entry.
    /// Tên binary (lệnh), dùng cho keg và entry PATH.
    pub name: String,
    /// Release version of the binary.
    /// Version phát hành của binary.
    pub version: String,
    /// Target triple the artifact was built for (`aarch64-apple-darwin`, …).
    /// Triple target của artifact (`aarch64-apple-darwin`, …).
    pub target: String,
    /// HTTPS download location.
    /// Địa chỉ tải HTTPS.
    pub url: String,
    /// BLAKE3 hex digest of the artifact bytes.
    /// Digest BLAKE3 hex của bytes artifact.
    pub blake3: String,
    /// Whether the artifact is an executable (affects keg sealing).
    /// Artifact có phải executable không (ảnh hưởng seal keg).
    #[serde(default = "default_executable")]
    pub executable: bool,
}

fn default_executable() -> bool {
    true
}

/// Signed manifest envelope.
/// Envelope manifest có chữ ký.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BinaryManifest {
    /// Signed payload.
    /// Payload được ký.
    pub entry: BinaryEntry,
    /// Base64 Ed25519 signature over [`manifest_message`].
    /// Chữ ký Ed25519 Base64 trên [`manifest_message`].
    pub signature: String,
    /// Base64 Ed25519 public key that produced the signature.
    /// Public key Ed25519 Base64 đã tạo chữ ký.
    pub public_key: String,
}

/// Canonical signed message for a manifest entry.
/// Thông điệp ký canonical cho một mục manifest.
///
/// Every field is included so a tampered URL or digest invalidates the
/// signature instead of being silently accepted.
/// (Mọi trường đều nằm trong message nên URL hoặc digest bị sửa sẽ làm chữ ký
/// mất hiệu lực thay vì được chấp nhận lặng lẽ.)
pub fn manifest_message(entry: &BinaryEntry) -> String {
    format!(
        "{MANIFEST_DOMAIN}\n{}\n{}\n{}\n{}\n{}\n{}",
        entry.name, entry.version, entry.target, entry.url, entry.blake3, entry.executable
    )
}

/// Verify a manifest against pinned trust keys.
/// Verify manifest với các key tin cậy đã pin.
///
/// Fails closed when no trust key is supplied: an unpinned manifest is an
/// unauthenticated download and must never be installable.
/// (Fail-closed khi không có key tin cậy: manifest không pin là tải chưa xác
/// thực và không bao giờ được cài.)
pub fn verify_manifest(manifest: &BinaryManifest, trust_keys: &[String]) -> KegResult<()> {
    if trust_keys.is_empty() {
        return Err(KegError::ManifestUntrusted {
            detail: "no trust key supplied; pass --trust-key or set MGC_LIB_TRUST_KEY".to_string(),
        });
    }
    if !trust_keys.iter().any(|key| key == &manifest.public_key) {
        return Err(KegError::ManifestUntrusted {
            detail: "manifest signed by a key that is not pinned".to_string(),
        });
    }
    let public_key =
        mgc_crypto::Ed25519PublicKey::from_base64(&manifest.public_key).map_err(|error| {
            KegError::ManifestUntrusted {
                detail: format!("manifest public key is not a valid Ed25519 key: {error}"),
            }
        })?;
    let signature =
        mgc_crypto::Ed25519Signature::from_base64(&manifest.signature).map_err(|error| {
            KegError::ManifestUntrusted {
                detail: format!("manifest signature is not valid base64: {error}"),
            }
        })?;
    mgc_crypto::ed25519_signer::verify_signature(
        &public_key,
        manifest_message(&manifest.entry).as_bytes(),
        &signature,
    )
    .map_err(|error| KegError::ManifestUntrusted {
        detail: format!("manifest signature does not verify: {error}"),
    })
}

/// Read and verify a manifest from disk.
/// Đọc và verify manifest từ đĩa.
pub fn load_manifest(path: &Path, trust_keys: &[String]) -> KegResult<BinaryManifest> {
    let text = std::fs::read_to_string(path).map_err(|error| {
        KegError::io(
            "read binary manifest",
            path,
            std::io::Error::other(error.to_string()),
        )
    })?;
    let manifest: BinaryManifest =
        serde_json::from_str(&text).map_err(|error| KegError::ManifestUntrusted {
            detail: format!("manifest is not valid JSON: {error}"),
        })?;
    verify_manifest(&manifest, trust_keys)?;
    Ok(manifest)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
