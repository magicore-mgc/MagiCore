//! Tree Hash owner wiring.
//! Wiring owner tree hash.

#[path = "tree_hash.rs"]
mod implementation;

pub(super) use implementation::hash_source_files;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
