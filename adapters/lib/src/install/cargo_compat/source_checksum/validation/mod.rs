//! Validation owner wiring.
//! Wiring owner validation.

#[path = "validation.rs"]
mod implementation;

pub(in crate::install::cargo_compat) use implementation::validate_sha256;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
