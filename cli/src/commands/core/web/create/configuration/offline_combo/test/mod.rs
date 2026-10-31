//! Offline-combo test wiring.
//! Wiring test combo offline.

#[path = "compose.rs"]
mod compose;

#[cfg(not(windows))]
#[path = "native_backend.rs"]
mod native_backend;
