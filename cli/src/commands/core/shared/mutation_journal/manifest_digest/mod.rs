//! Manifest digest module wiring.
//! Wiring module digest manifest.

#[path = "manifest_digest.rs"]
mod implementation;

pub(crate) use implementation::manifest_canonical_digest;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
