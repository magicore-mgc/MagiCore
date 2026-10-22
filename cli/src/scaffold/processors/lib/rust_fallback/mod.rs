//! Rust fallback scaffold owner wiring.
//! Wiring owner scaffold Rust fallback.

#[path = "rust_fallback.rs"]
mod implementation;
pub(super) use implementation::write;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
