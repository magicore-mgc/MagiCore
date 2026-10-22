//! Manifest hint wiring — nối parser theo định dạng manifest.
//! Wiring core hint — nối parser theo định dạng manifest.

#[path = "manifest_hints.rs"]
mod manifest_hints;

pub(in crate::dispatch) use manifest_hints::read;
