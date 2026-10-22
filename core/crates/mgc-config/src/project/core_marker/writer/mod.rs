//! Core marker writer wiring and tests.
//! Wiring và test writer core marker.
mod writer;
pub(super) use writer::{ensure_core_marker_at, write_core_marker, write_core_marker_at};

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
