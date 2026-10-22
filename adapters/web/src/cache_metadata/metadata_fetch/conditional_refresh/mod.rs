//! Conditional-registry-refresh owner wiring.
//! Wiring owner refresh registry có điều kiện.

mod conditional_refresh;

pub(in crate::cache_metadata) use conditional_refresh::refresh_cached_metadata;
