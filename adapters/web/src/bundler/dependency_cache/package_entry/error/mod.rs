//! Package-entry error owner wiring.
//! Nối owner error package-entry.

#[path = "error.rs"]
mod implementation;

pub(in crate::bundler::dependency_cache) use implementation::PackageEntryError;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
