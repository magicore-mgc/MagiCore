//! Signed read-only exception manifest — policy data có ký cho exception đọc.
//!
//! Narrow, versioned policy data living outside the Rust binary (Spec rev4
//! §5): exact invocations only, never a general mutation grant. Signature
//! proves *who published* the data, not that the policy is correct.
//! (Policy data hẹp, versioned, ngoài binary Rust (Spec rev4 §5): chỉ
//! invocation chính xác, không bao giờ cấp mutation chung. Chữ ký chứng
//! minh *ai phát hành* dữ liệu, không chứng minh policy đúng.)
//!
//! Offline rule: a verified, unexpired cached manifest stays usable until
//! expiry; expiry bounds offline revocation delay. Revocation takes effect
//! when a signed revocation snapshot is received.
//! (Luật offline: manifest đã xác minh còn hạn dùng được đến expiry; expiry
//! là giới hạn tối đa cho độ trễ thu hồi offline. Thu hồi có hiệu lực khi
//! nhận snapshot thu hồi có chữ ký.)

use serde::{Deserialize, Serialize};

/// Manifest schema this code understands — schema manifest code này hiểu.
pub const POLICY_MANIFEST_SCHEMA: &str = "policy-exception-v2";

/// One exact read-only exception — một ngoại lệ chỉ đọc chính xác.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestEntry {
    /// Program basename, lowercase — basename chương trình, lowercase.
    pub program: String,
    /// Exact argument list — danh sách tham số chính xác.
    pub args_exact: Vec<String>,
    /// Framework the exception was verified for (e.g. `"next"`).
    /// Framework đã xác minh exception (vd `"next"`).
    pub framework: String,
    /// Framework version verified (e.g. `"16.3.8"`).
    /// Version framework đã xác minh (vd `"16.3.8"`).
    pub framework_version: String,
    /// Operating system (e.g. `"darwin"`) — hệ điều hành (vd `"darwin"`).
    pub os: String,
    /// Operation (e.g. `"dev"`) — thao tác (vd `"dev"`).
    pub operation: String,
    /// Why this invocation is needed — vì sao cần invocation này.
    pub reason: String,
    /// Expected observable output — output quan sát được kỳ vọng.
    pub expected: String,
}

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

/// Verification failure — lỗi xác minh.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ManifestError {
    /// Unknown schema version — phiên bản schema không nhận diện.
    UnknownSchema(String),
    /// Signing key is not in the trusted set — khóa ký không được tin.
    UntrustedKey,
    /// Signing key was revoked — khóa ký đã bị thu hồi.
    RevokedKey,
    /// A revoked key id is not a canonical lowercase BLAKE3 hex id.
    /// Key ID bị thu hồi không phải BLAKE3 hex chữ thường chuẩn.
    InvalidRevocationKey,
    /// Bad signature — chữ ký sai.
    BadSignature,
    /// Past expiry — đã hết hạn.
    Expired,
    /// Manifest was issued in the future — manifest chưa tới thời điểm hiệu lực.
    NotYetValid,
    /// Expiry must be strictly later than issue time — expires_at phải sau issued_at.
    InvalidTimeWindow,
}

/// Stable canonical bytes covered by the signature (field order fixed,
/// entries in document order — the signer and verifier must agree).
/// Bytes canonical ổn định được chữ ký phủ (thứ tự field cố định,
/// entries theo thứ tự tài liệu — bên ký và bên verify phải khớp).
pub fn canonical_bytes(manifest: &PolicyManifest) -> String {
    let mut out = String::from("MagiCore.PolicyException.v2\n");
    push_canonical_field(&mut out, &manifest.schema_version);
    push_canonical_field(&mut out, &manifest.id);
    push_canonical_field(&mut out, &manifest.issued_at.to_string());
    push_canonical_field(&mut out, &manifest.expires_at.to_string());
    push_canonical_field(&mut out, &manifest.key_id);
    let mut revoked = manifest.revoked_keys.clone();
    revoked.sort();
    push_canonical_field(&mut out, &revoked.len().to_string());
    for key in &revoked {
        push_canonical_field(&mut out, key);
    }
    push_canonical_field(&mut out, &manifest.entries.len().to_string());
    for entry in &manifest.entries {
        push_canonical_field(&mut out, &entry.program);
        push_canonical_field(&mut out, &entry.args_exact.len().to_string());
        for arg in &entry.args_exact {
            push_canonical_field(&mut out, arg);
        }
        push_canonical_field(&mut out, &entry.framework);
        push_canonical_field(&mut out, &entry.framework_version);
        push_canonical_field(&mut out, &entry.os);
        push_canonical_field(&mut out, &entry.operation);
        push_canonical_field(&mut out, &entry.reason);
        push_canonical_field(&mut out, &entry.expected);
    }
    out
}

/// Append one UTF-8 byte-length-prefixed field; length prefixes keep fields
/// and argument vectors unambiguous even when values contain delimiters.
/// Nối field có tiền tố độ dài byte UTF-8; không thể nhập nhằng do delimiter.
fn push_canonical_field(out: &mut String, value: &str) {
    out.push_str(&value.len().to_string());
    out.push(':');
    out.push_str(value);
}

/// Current Unix time in seconds — giờ unix hiện tại theo giây.
pub fn now_unix_secs() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

/// Verify schema, key trust, external revocation, signature and expiry.
/// Selection separately applies revocations carried by this signed snapshot.
/// Xác minh schema, trust, revocation bên ngoài, chữ ký và hạn. Bước chọn
/// manifest áp dụng riêng danh sách thu hồi có trong snapshot đã ký này.
pub fn verify_manifest(
    manifest: &PolicyManifest,
    trusted_key_ids: &[String],
    revoked_key_ids: &[String],
    now_secs: i64,
) -> Result<(), ManifestError> {
    // Unknown schema fails closed (never parse-and-hope).
    // (Schema lạ fail-closed (không parse-cầu-may).)
    if manifest.schema_version != POLICY_MANIFEST_SCHEMA {
        return Err(ManifestError::UnknownSchema(
            manifest.schema_version.clone(),
        ));
    }
    if !trusted_key_ids.contains(&manifest.key_id) {
        return Err(ManifestError::UntrustedKey);
    }
    if manifest.expires_at <= manifest.issued_at {
        return Err(ManifestError::InvalidTimeWindow);
    }
    if now_secs < manifest.issued_at {
        return Err(ManifestError::NotYetValid);
    }
    if revoked_key_ids.contains(&manifest.key_id) {
        return Err(ManifestError::RevokedKey);
    }
    let public_key = mgc_crypto::ed25519_signer::Ed25519PublicKey::from_base64(&manifest.key_id)
        .map_err(|_| ManifestError::UntrustedKey)?;
    let signature = mgc_crypto::ed25519_signer::Ed25519Signature::from_base64(&manifest.signature)
        .map_err(|_| ManifestError::BadSignature)?;
    mgc_crypto::ed25519_signer::verify_string_signature(
        &public_key,
        &canonical_bytes(manifest),
        &signature,
    )
    .map_err(|_| ManifestError::BadSignature)?;
    if now_secs >= manifest.expires_at {
        return Err(ManifestError::Expired);
    }
    if manifest
        .revoked_keys
        .iter()
        .any(|key| !is_canonical_key_id_hex(key))
    {
        return Err(ManifestError::InvalidRevocationKey);
    }
    Ok(())
}

/// Revocation IDs use the same 16-character lowercase BLAKE3 hex shape as
/// `MGC_POLICY_TRUST_KEYS`.
/// (ID thu hồi dùng cùng dạng BLAKE3 hex chữ thường 16 ký tự với trust keys.)
fn is_canonical_key_id_hex(value: &str) -> bool {
    value.len() == 16
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

/// Find the exact exception for an invocation in a verified manifest.
/// Returns `None` unless program, full args, framework, version, OS and
/// operation all match one entry (Spec rev4 §5/§7 profile binding is
/// enforced by the caller through [`crate::policy::resolve`]).
/// (Tìm exception chính xác cho invocation trong manifest đã xác minh.
/// Trả `None` trừ khi program, đủ args, framework, version, OS và operation
/// đều khớp một entry (ràng buộc profile ở Spec rev4 §5/§7 do caller thực
/// thi qua [`crate::policy::resolve`]).)
pub fn find_exception(
    manifest: &PolicyManifest,
    program: &str,
    args: &[String],
    framework: &str,
    framework_version: &str,
    os: &str,
    operation: &str,
) -> Option<crate::policy::ReadOnlyException> {
    let tool = crate::policy::tool_basename(program);
    manifest
        .entries
        .iter()
        .find(|entry| {
            entry.program == tool
                && entry.args_exact == args
                && entry.framework == framework
                && entry.framework_version == framework_version
                && entry.os == os
                && entry.operation == operation
        })
        .map(|entry| crate::policy::ReadOnlyException {
            program: entry.program.clone(),
            args_exact: entry.args_exact.clone(),
            unexpired: true,
            answer: None,
        })
}

/// Relative cache path for the manifest under a data root.
/// Đường dẫn cache tương đối cho manifest dưới data root.
pub fn cache_rel_path() -> std::path::PathBuf {
    std::path::PathBuf::from("policy/exceptions.json")
}

/// BLAKE3 key id (first 8 bytes, hex) of a base64 Ed25519 public key —
/// the same id format as `mgc.toml [trust] keys` and lockfile signers.
/// Key id BLAKE3 (8 byte đầu, hex) của khóa công khai Ed25519 base64 —
/// cùng định dạng id với `mgc.toml [trust] keys` và signer lockfile.
pub fn key_id_hex(public_key_b64: &str) -> Option<String> {
    let key = mgc_crypto::ed25519_signer::Ed25519PublicKey::from_base64(public_key_b64).ok()?;
    let digest = mgc_crypto::blake3_signer::Blake3Hasher::hash_bytes(&key.0);
    Some(digest.to_hex().chars().take(16).collect())
}

/// Select the active manifest from rotation candidates (Spec rev4 §5/§16.7).
/// Newest valid `issued_at` wins; revoked keys are unioned across all
/// verified snapshots, so a newer revocation snapshot invalidates older
/// manifests signed by the revoked key. Unverifiable candidates are
/// skipped (the base native-deny layers stay enforced without them).
/// (Chọn manifest hiệu lực từ các bản rotation (Spec rev4 §16.7). Bản hợp lệ
/// `issued_at` mới nhất thắng; khóa thu hồi được hợp nhất qua mọi snapshot
/// đã xác minh, nên snapshot thu hồi mới hơn vô hiệu manifest cũ do khóa đó
/// ký. Bản không xác minh được thì bỏ qua (các lớp native-deny gốc vẫn thực
/// thi khi không có chúng).)
pub fn select_active_manifest(
    manifests: Vec<PolicyManifest>,
    trusted_hex_ids: &[String],
    extra_revoked_hex_ids: &[String],
    now_secs: i64,
) -> Option<PolicyManifest> {
    // Verify signed snapshots at the last second of their declared validity.
    // This keeps authentic revocations effective after the exception payload
    // expires, while expired payloads themselves cannot supply exceptions.
    // Xác minh snapshot ký tại giây cuối của hạn đã khai báo. Revocation xác
    // thực vẫn hiệu lực sau expiry, nhưng payload hết hạn không cấp exception.
    let mut verified: Vec<PolicyManifest> = Vec::new();
    let mut revoked: Vec<String> = extra_revoked_hex_ids.to_vec();
    for manifest in manifests {
        let hex = match key_id_hex(&manifest.key_id) {
            Some(hex) => hex,
            None => continue,
        };
        if !trusted_hex_ids.contains(&hex) {
            continue;
        }
        let signature_time = manifest.expires_at.saturating_sub(1);
        if verify_manifest(
            &manifest,
            std::slice::from_ref(&manifest.key_id),
            &[],
            signature_time,
        )
        .is_err()
        {
            continue;
        }
        if manifest.issued_at <= now_secs {
            revoked.extend(manifest.revoked_keys.iter().cloned());
        }
        if manifest.issued_at <= now_secs && now_secs < manifest.expires_at {
            verified.push(manifest);
        }
    }
    // Drop revoked signers and pick the newest active payload.
    // Loại signer bị thu hồi rồi chọn payload còn hạn mới nhất.
    verified
        .into_iter()
        .filter(|manifest| {
            key_id_hex(&manifest.key_id)
                .map(|hex| !revoked.contains(&hex))
                .unwrap_or(false)
        })
        .max_by_key(|manifest| manifest.issued_at)
}
