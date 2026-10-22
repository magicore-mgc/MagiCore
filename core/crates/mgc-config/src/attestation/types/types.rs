//! Attestation record types / Các type record attestation.

use serde::{Deserialize, Serialize};

/// A previous signing key kept for verifying older attestations.
/// Key ký cũ giữ lại để verify chứng thực cũ.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreviousKey {
    /// Key id (BLAKE3-8 of the public key, keyring convention).
    /// Key id (BLAKE3-8 của public key, theo quy ước keyring).
    pub key_id: String,
    /// Base64 Ed25519 public key.
    /// Public key Ed25519 ở dạng Base64.
    pub public_key: String,
}

/// Core-identity attestation — Chứng thực identity core.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Attestation {
    /// Canonical core name (`web`, `clo`, …).
    /// Tên core canonical (`web`, `clo`, …).
    pub core: String,
    /// BLAKE3 hex of the exact `mgc.toml` bytes at attest time.
    /// BLAKE3 hex của đúng bytes `mgc.toml` tại thời điểm attest.
    pub config_digest: String,
    /// Absolute project path the attestation was made for.
    /// Đường dẫn tuyệt đối của project được chứng thực.
    pub project_path: String,
    /// Signing key id.
    /// Key id dùng để ký.
    pub key_id: String,
    /// Base64 Ed25519 public key (verify without the keyring).
    /// Public key Ed25519 dạng Base64 (verify không cần keyring).
    pub public_key: String,
    /// Rotated-out keys, newest first (at most one generation kept).
    /// Các key đã rotation, mới nhất trước (giữ tối đa một thế hệ).
    #[serde(default)]
    pub prev_keys: Vec<PreviousKey>,
    /// RFC 3339 attest time.
    /// Thời điểm attest theo RFC 3339.
    pub attested_at: String,
    /// mgc version that attested.
    /// Version mgc đã thực hiện attest.
    pub mgc_version: String,
    /// Base64 Ed25519 signature over the canonical message.
    /// Chữ ký Ed25519 dạng Base64 trên canonical message.
    pub signature: String,
}

/// Attestation check outcome — Kết quả kiểm tra chứng thực.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttestationStatus {
    /// No attestation slot for this project.
    /// Project chưa có slot chứng thực.
    Absent,
    /// Signature and digest match the live project.
    /// Chữ ký và digest khớp với project hiện tại.
    Valid,
    /// An attestation exists but does not match (reason included).
    /// Có chứng thực nhưng không khớp; kèm lý do.
    Mismatch(String),
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
