//! Wires offline Rust cache preflight and replay.
//! Nối preflight và replay cache Rust offline.

mod offline_install;
pub(crate) use self::offline_install::{install_rust_cached, preflight_rust_artifact_cache};
