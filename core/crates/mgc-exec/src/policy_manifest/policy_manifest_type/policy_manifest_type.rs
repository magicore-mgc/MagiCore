use crate::policy_manifest::ManifestEntry;
use serde::{Deserialize, Serialize};

/// Signed exception manifest — manifest exception có ký.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyManifest {
    /// Schema version — must equal [`POLICY_MANIFEST_SCHEMA`].
    /// Phiên bản schema — phải bằng [`POLICY_MANIFEST_SCHEMA`].
    pub schema_version: String,
    /// Manifest id (e.g. `"2026-10-07-next-darwin"`) — id manifest.
    pub id: String,
    /// Unix seconds when issued — giây unix lúc phát hành.
    pub issued_at: i64,
    /// Unix seconds when this manifest stops being usable.
    /// Giây unix khi manifest hết dùng được.
    pub expires_at: i64,
    /// Base64 Ed25519 public key that signed this manifest.
    /// Khóa công khai Ed25519 base64 đã ký manifest.
    pub key_id: String,
    /// Key ids revoked by this snapshot (rotation support).
    /// Các key id bị thu hồi bởi snapshot này (hỗ trợ rotation).
    #[serde(default)]
    pub revoked_keys: Vec<String>,
    /// Exact exceptions — các exception chính xác.
    pub entries: Vec<ManifestEntry>,
    /// Base64 Ed25519 signature over [`PolicyManifest::canonical_bytes`].
    /// Chữ ký Ed25519 base64 trên [`PolicyManifest::canonical_bytes`].
    pub signature: String,
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
