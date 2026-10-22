//! Cached-metadata reuse-policy owner wiring.
//! Wiring owner chính sách tái sử dụng metadata đã cache.

mod cached_policy;

pub(in crate::cache_metadata) use cached_policy::reuse_without_refresh;
