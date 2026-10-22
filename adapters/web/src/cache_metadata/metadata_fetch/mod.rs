//! Metadata-fetch responsibility-owner wiring.
//! Wiring các owner trách nhiệm tải metadata.

mod cache_lookup;
mod cached_policy;
mod conditional_refresh;
mod metadata_fetch;
mod network_fallback;

pub use self::metadata_fetch::{load_metadata_by_name_with_fallback, load_metadata_with_fallback};
pub(super) use cache_lookup::load_cached_metadata;
pub(super) use cached_policy::reuse_without_refresh;
pub(super) use conditional_refresh::refresh_cached_metadata;
pub(super) use network_fallback::fetch_with_stale_fallback;
