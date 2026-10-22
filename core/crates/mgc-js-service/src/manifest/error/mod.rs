//! Manifest-local error exports and nearest tests.
//! Export lỗi cục bộ manifest và test gần owner.

mod error;

pub(super) use error::ManifestError;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
