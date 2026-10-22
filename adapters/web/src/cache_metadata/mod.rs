//! Public compatibility facade for Web package metadata caching.
//! Facade tương thích public cho cache metadata package Web.

mod metadata_cache;
mod metadata_fetch;
mod metadata_policy;
mod metadata_records;

pub use self::metadata_cache::MetadataCache;
pub use self::metadata_fetch::{load_metadata_by_name_with_fallback, load_metadata_with_fallback};
pub use self::metadata_policy::{
    metadata_concurrency_limit, metadata_max_stale_fallback_secs, metadata_record_is_fresh,
    metadata_record_is_usable_stale, metadata_record_retry_deferred, metadata_stale_retry_ttl_secs,
    metadata_ttl_secs, next_stale_retry_after,
};
pub use self::metadata_records::{CachedMetadataEnvelope, CachedMetadataRecord};
