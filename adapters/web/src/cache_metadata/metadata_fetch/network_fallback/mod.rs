//! Full-fetch and stale-fallback owner wiring.
//! Wiring owner fetch đầy đủ và fallback stale.

mod network_fallback;

pub(in crate::cache_metadata) use network_fallback::fetch_with_stale_fallback;
