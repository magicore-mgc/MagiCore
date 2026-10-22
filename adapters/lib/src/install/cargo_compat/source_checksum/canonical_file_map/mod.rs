//! Canonical File Map owner wiring.
//! Wiring owner canonical file map.

#[path = "canonical_file_map.rs"]
mod implementation;

pub(super) use implementation::source_files_match_archive;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
