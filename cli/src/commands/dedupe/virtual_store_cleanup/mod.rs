//! Virtual-store cleanup owner wiring — exposes post-verification GC.
//! Wiring owner dọn virtual store — export GC sau xác minh.
mod error;
mod measurement;
mod root;
mod virtual_store_cleanup;
pub(super) use virtual_store_cleanup::{GcReport, cleanup_unreferenced_vstore};
