//! Bounded registry response decoding.
//! Giải mã response registry có giới hạn.

mod bundle_reader;
mod error;

pub(super) use bundle_reader::read_bundle;
pub(super) use error::BundleReadError;

#[cfg(test)]
#[path = "test/size_limit.rs"]
mod tests;
