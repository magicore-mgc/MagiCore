//! Public merge-conflict type owner wiring.
//! Wiring owner type merge-conflict công khai.

#[path = "merge_conflict.rs"]
mod implementation;
pub use self::implementation::MergeConflict;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
