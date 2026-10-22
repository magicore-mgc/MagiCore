//! Exports the OCI blob error contract and its focused tests.
//! Export hợp đồng lỗi OCI blob và test tương ứng.

mod error;

pub(super) use error::{OciBlobError, OciBlobResult};

#[cfg(test)]
#[path = "test/error.rs"]
mod tests;
