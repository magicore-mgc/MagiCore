//! Own freshness, stale eligibility, retry, and concurrency policy.
//! Quản lý chính sách tươi/cũ, thời gian thử lại và đồng thời.

use crate::cache::current_unix_secs;
use crate::cache_metadata::CachedMetadataRecord;

pub fn metadata_record_is_fresh(record: &CachedMetadataRecord) -> bool {
    if record.fetched_at == 0 {
        return false;
    }
    current_unix_secs().saturating_sub(record.fetched_at) <= metadata_ttl_secs()
}

pub fn metadata_record_retry_deferred(record: &CachedMetadataRecord) -> bool {
    record
        .stale_retry_after
        .is_some_and(|retry_after| retry_after > current_unix_secs())
}

pub fn metadata_record_is_usable_stale(record: &CachedMetadataRecord) -> bool {
    if record.fetched_at == 0 {
        return true;
    }
    current_unix_secs().saturating_sub(record.fetched_at) <= metadata_max_stale_fallback_secs()
}

pub fn metadata_ttl_secs() -> u64 {
    std::env::var("MAGICORE_WEB_METADATA_TTL_SECS")
        .ok()
        .and_then(|raw| raw.trim().parse::<u64>().ok())
        .unwrap_or(6 * 60 * 60)
}

pub fn metadata_max_stale_fallback_secs() -> u64 {
    std::env::var("MAGICORE_WEB_METADATA_MAX_STALE_SECS")
        .ok()
        .and_then(|raw| raw.trim().parse::<u64>().ok())
        .filter(|ttl| *ttl > 0)
        .unwrap_or(24 * 60 * 60)
}

pub fn metadata_stale_retry_ttl_secs() -> u64 {
    std::env::var("MAGICORE_WEB_METADATA_STALE_RETRY_TTL_SECS")
        .ok()
        .and_then(|raw| raw.trim().parse::<u64>().ok())
        .filter(|ttl| *ttl > 0)
        .unwrap_or(30)
}

pub fn metadata_concurrency_limit() -> usize {
    std::env::var("MAGICORE_WEB_METADATA_CONCURRENCY")
        .ok()
        .and_then(|raw| raw.trim().parse::<usize>().ok())
        .filter(|limit| *limit > 0)
        .unwrap_or(24)
}

pub fn next_stale_retry_after() -> u64 {
    current_unix_secs().saturating_add(metadata_stale_retry_ttl_secs())
}
