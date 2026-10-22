//! Read a trusted cached snapshot and retain its revocation history.
//! Đọc snapshot cache tin cậy và giữ lịch sử thu hồi.

use std::io::Read;
use std::path::Path;

use crate::commands::policy::manifest_cache::MAX_POLICY_MANIFEST_BYTES;

/// Read a trusted cached snapshot even after its exception payload expires.
/// Its signature and revocation history still constrain future cache updates.
/// (Đọc snapshot cache đã tin cậy kể cả khi payload exception hết hạn. Chữ ký
/// và lịch sử thu hồi vẫn ràng buộc các lần cập nhật cache sau.)
pub(in crate::commands::policy::manifest_cache) fn read_verified_cached_manifest(
    cache_path: &Path,
    trusted_hex_ids: &[String],
) -> Option<mgc_exec::policy_manifest::PolicyManifest> {
    let file = std::fs::File::open(cache_path).ok()?;
    let mut bytes = Vec::new();
    let size = file
        .take((MAX_POLICY_MANIFEST_BYTES + 1) as u64)
        .read_to_end(&mut bytes)
        .ok()?;
    if size > MAX_POLICY_MANIFEST_BYTES {
        return None;
    }
    let manifest: mgc_exec::policy_manifest::PolicyManifest =
        serde_json::from_slice(&bytes).ok()?;
    let signer_hex = mgc_exec::policy_manifest::key_id_hex(&manifest.key_id)?;
    if !trusted_hex_ids.contains(&signer_hex) {
        return None;
    }
    let verification_time = manifest.expires_at.saturating_sub(1);
    mgc_exec::policy_manifest::verify_manifest(
        &manifest,
        std::slice::from_ref(&manifest.key_id),
        &[],
        verification_time,
    )
    .ok()?;
    Some(manifest)
}

#[cfg(test)]
#[path = "test/verified_cached_manifest.rs"]
mod tests;
