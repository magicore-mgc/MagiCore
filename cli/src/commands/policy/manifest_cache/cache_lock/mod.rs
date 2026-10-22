//! Cache writer lock owner wiring.
//! Nối owner khóa writer của cache.

mod cache_lock;

pub(super) use cache_lock::lock_policy_manifest_cache;
