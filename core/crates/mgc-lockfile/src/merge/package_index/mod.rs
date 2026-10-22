//! Package indexing owner wiring.
//! Wiring owner index package.

#[path = "package_index.rs"]
mod implementation;
pub(super) use self::implementation::{PackageIdentity, PkgMap, index_pkgs};

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
