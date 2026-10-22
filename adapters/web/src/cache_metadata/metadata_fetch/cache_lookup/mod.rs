//! Cache-lookup owner wiring.
//! Wiring owner tra cứu cache metadata.

mod cache_lookup;

pub(in crate::cache_metadata) use cache_lookup::load_cached_metadata;
