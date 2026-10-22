//! Strict package-link error owner wiring.
//! Wiring owner lỗi package-link strict.

#[path = "error.rs"]
mod implementation;

pub(super) use implementation::{LinkTreeError, LinkTreeOperation};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
