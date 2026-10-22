//! Exact Gradle pin-collapse wiring.
//! Nối xử lý gộp pin Gradle chính xác.

#[path = "collapse.rs"]
mod implementation;

pub(super) use implementation::collapse_entries;
