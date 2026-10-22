//! Local fixture facade for compiled-cache integration tests.
//! Facade fixture cục bộ cho integration test compiled-cache.

#[path = "support.rs"]
mod support;
pub(crate) use support::{SRC, entry_path, module, out_digest, scan_temps, tmp_store_dir};
