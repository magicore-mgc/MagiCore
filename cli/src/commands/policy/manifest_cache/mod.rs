//! Signed manifest cache owner wiring.
//! Nối owner cache manifest đã ký.

mod cache_lock;
mod error;
mod manifest_cache;
mod verified_cached_manifest;

pub(in crate::commands::policy) use error::PolicyManifestCacheError;
pub(in crate::commands::policy) use manifest_cache::{MAX_POLICY_MANIFEST_BYTES, verify_and_cache};
