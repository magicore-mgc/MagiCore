//! Verify and atomically update the shared signed manifest cache.
//! Xác minh và cập nhật atomically cache manifest ký dùng chung.

use std::collections::BTreeSet;
use std::path::Path;

use super::error::{PolicyManifestCacheError, PolicyManifestCacheResult};

pub(in crate::commands::policy) const MAX_POLICY_MANIFEST_BYTES: usize = 1024 * 1024;

/// Verify fetched bytes and atomically replace the cache (shared by URL and
/// OCI channels — one verification path, no per-channel drift).
/// (Xác minh bytes tải về và thay atomically cache (dùng chung cho kênh URL
/// và OCI — một đường xác minh, không lệch theo kênh).)
pub(in crate::commands::policy) fn verify_and_cache(
    bytes: &[u8],
    trusted_hex_ids: &[String],
    cache_path: &Path,
) -> PolicyManifestCacheResult<mgc_exec::policy_manifest::PolicyManifest> {
    if bytes.len() > MAX_POLICY_MANIFEST_BYTES {
        return Err(PolicyManifestCacheError::OversizedManifest);
    }
    let manifest: mgc_exec::policy_manifest::PolicyManifest =
        serde_json::from_slice(bytes).map_err(PolicyManifestCacheError::InvalidJson)?;
    let now = mgc_exec::policy_manifest::now_unix_secs();
    let selected = mgc_exec::policy_manifest::select_active_manifest(
        vec![manifest],
        trusted_hex_ids,
        &[],
        now,
    )
    .ok_or(PolicyManifestCacheError::UnverifiedManifest)?;
    if let Some(parent) = cache_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    // Serialize the complete read/compare/write transaction across MGC
    // processes. (Tuần tự hóa trọn giao dịch đọc/so sánh/ghi giữa các process.)
    let _cache_lock = super::cache_lock::lock_policy_manifest_cache(cache_path)?;
    if let Some(previous) =
        super::verified_cached_manifest::read_verified_cached_manifest(cache_path, trusted_hex_ids)
    {
        if selected.issued_at < previous.issued_at {
            return Err(PolicyManifestCacheError::UpdateRejected(
                "incoming snapshot is older than the verified cache".to_string(),
            ));
        }
        if selected.issued_at == previous.issued_at {
            if mgc_exec::policy_manifest::canonical_bytes(&selected)
                == mgc_exec::policy_manifest::canonical_bytes(&previous)
            {
                return Ok(previous);
            }
            return Err(PolicyManifestCacheError::UpdateRejected(
                "different snapshots share the cached issued_at timestamp".to_string(),
            ));
        }

        let previous_revoked = previous
            .revoked_keys
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        let incoming_revoked = selected
            .revoked_keys
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        if let Some(removed_key) = previous_revoked.difference(&incoming_revoked).next() {
            return Err(PolicyManifestCacheError::UpdateRejected(format!(
                "newer snapshot dropped previously revoked key '{removed_key}'"
            )));
        }
    }
    let tmp = cache_path.with_extension("json.tmp");
    let serialized =
        serde_json::to_vec_pretty(&selected).map_err(PolicyManifestCacheError::Serialization)?;
    std::fs::write(&tmp, serialized)?;
    std::fs::rename(&tmp, cache_path)?;
    Ok(selected)
}

#[cfg(test)]
#[path = "test/cache.rs"]
mod tests;
