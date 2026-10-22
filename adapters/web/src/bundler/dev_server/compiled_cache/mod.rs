//! Compiled Cache owner wiring — nối owner compiled cache.
#[path = "compiled_cache.rs"]
mod implementation;
pub(super) use self::implementation::{cached_raw_js, save_raw_js};
