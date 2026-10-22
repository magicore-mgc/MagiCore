//! Wires Rust registry archive cache helpers.
//! Nối các helper cache archive registry Rust.

mod artifact_cache;
pub(crate) use self::artifact_cache::{cargo_archive_path, read_cached_rust_artifact};
