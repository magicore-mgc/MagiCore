//! macOS process scan facade — export macOS process snapshots.
//! Facade quét process macOS — export snapshot process macOS.

#[path = "macos.rs"]
mod implementation;
pub(in crate::run::implementation) use implementation::{
    macos_process_snapshot, macos_process_snapshots,
};
