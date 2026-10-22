//! Core identity module wiring and exports.
//! Wiring và export module identity core.
mod identity;
pub(super) use identity::{canonical_core, is_known_core, read_core_marker_identity};

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
