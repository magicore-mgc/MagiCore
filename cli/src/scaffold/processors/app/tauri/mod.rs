//! Tauri owner wiring.
//! Wiring owner Tauri.

#[path = "tauri.rs"]
mod implementation;
pub(super) use implementation::write;
