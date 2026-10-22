//! Metadata error exports — xuất lỗi typed của owner metadata.

mod error;

pub(super) use error::MetadataError;

#[cfg(test)]
#[path = "test/error.rs"]
mod tests;
