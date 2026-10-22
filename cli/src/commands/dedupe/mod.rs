//! Dedupe command module wiring — preserves the public CLI command and payload paths.
//! Wiring module lệnh dedupe — giữ nguyên API public của command và payload.
mod build_verification;
mod dedupe;
mod lockfile_merge;
mod report;
mod types;
mod virtual_store_cleanup;

pub use dedupe::run;
pub use types::{DedupeArgs, DedupeEntry, DedupeReport};
