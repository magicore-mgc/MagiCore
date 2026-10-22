//! Core marker reader wiring and tests.
//! Wiring và test reader core marker.
mod reader;
pub(super) use reader::read_core_marker;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
