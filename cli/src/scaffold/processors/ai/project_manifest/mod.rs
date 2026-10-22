//! Project Manifest output owner wiring.
//! Wiring owner ghi output project manifest.

#[path = "project_manifest.rs"]
mod implementation;
pub(super) use implementation::write;
