//! Rust-native Cargo source readiness owner wiring.
//! Wiring owner sẵn sàng source Cargo cho install Rust native.

#[path = "source_readiness.rs"]
mod implementation;

pub(crate) use implementation::{finalize_source, require_cargo_source_marker, source_root};

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
