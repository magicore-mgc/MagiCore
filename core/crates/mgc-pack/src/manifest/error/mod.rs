//! Local manifest sanitization error owner exports.
//! Export owner lỗi cục bộ cho sanitize manifest.

mod error;

pub(crate) use error::ManifestSanitizeError;

#[cfg(test)]
#[path = "test/error.rs"]
mod tests;
