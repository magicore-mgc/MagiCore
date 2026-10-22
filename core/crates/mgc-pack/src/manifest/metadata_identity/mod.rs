//! Manifest identity fallback and override owner.
//! Owner fallback và override định danh manifest.

mod metadata_identity;

pub(crate) use metadata_identity::{name_or, version_or};

#[cfg(test)]
#[path = "test/metadata_identity.rs"]
mod tests;
